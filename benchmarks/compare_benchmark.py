import json
from pathlib import Path
from statistics import mean, median
import math

def load_json(p: Path):
    if not p.exists():
        return None
    with p.open("r", encoding="utf-8") as f:
        return json.load(f)

def pick_time_value(entry: dict):
    """Robustly pick a numeric timing from an entry."""
    for key in ("mean_time_ms", "median_time_ms", "max_time_ms"):
        if key in entry:
            try:
                return float(entry[key]), entry.get("size")
            except Exception:
                pass
    # fallback
    for k, v in entry.items():
        if isinstance(v, (int, float)):
            if k in ("iterations", "size", "runs"):
                continue
            try:
                return float(v), entry.get("size")
            except Exception:
                pass
    return None, entry.get("size")

def build_map(entries):
    out = {}
    for e in entries:
        name = e.get("name") or e.get("id") or e.get("test") or None
        if not name:
            name = json.dumps(e, sort_keys=True)
        out[name] = e
    return out

def load_all_data(output_dir: Path):
    files = {
        "Rust (CPU)": output_dir / "rust_benchmark_cpu.json",
        "Rust (Serial)": output_dir / "rust_benchmark_cpu_serial.json",
        "Rust (GPU)": output_dir / "rust_benchmark_gpu.json",
        "R": output_dir / "r_benchmark.json"
    }
    
    data = {}
    for label, path in files.items():
        loaded = load_json(path)
        if loaded:
            # Flatten category structure: {category: [entries]} -> {name: entry}
            flat = {}
            for cat, entries in loaded.items():
                for entry in entries:
                    name = entry.get("name")
                    if name:
                        flat[name] = entry
            data[label] = flat
    return data

def main():
    repo_root = Path(__file__).resolve().parent
    workspace = repo_root
    for _ in range(6):
        if (workspace / "output").exists():
            break
        if workspace.parent == workspace:
            break
        workspace = workspace.parent
    out_dir = workspace / "output"
    
    data = load_all_data(out_dir)
    r_data = data.get("R")
    
    if not r_data:
        print("R baseline data not found or empty.")
        return

    # Collect all benchmark names
    all_names = set(r_data.keys())
    for label, d in data.items():
        if label != "R":
            all_names.update(d.keys())
            
    large_scale_benchmarks = {
        "scale_100000", "scale_1000000", "scale_1e+05",
        "scale_250000", "scale_500000", 
        "scale_2000000"
    }
            
    regular_names = sorted([n for n in all_names if n not in large_scale_benchmarks])
    large_scale_names = sorted([n for n in all_names if n in large_scale_benchmarks])
    sorted_names = regular_names + large_scale_names
    
    # Columns
    candidates = ["Rust (CPU)*", "Rust (GPU)"]
    
    # Print Table Header
    # Format: 
    # Name | R | Rust (CPU)* | Rust (GPU) |
    print(f"{'Name':<21} | {'R':^11} | {'Rust (CPU)*':^13} | {'Rust (GPU)':^9} |")
    print("-" * 65)

    for name in sorted_names:
        is_large_scale = name in large_scale_benchmarks
        display_name = f"{name}**" if is_large_scale else name

        # Baseline logic (R)
        base_col_str = "-"
        base_val = None
        
        # R is always the baseline here for non-large scale, or general comparison
        base_entry = r_data.get(name)
        if base_entry:
             base_val, _ = pick_time_value(base_entry)
             if base_val and base_val > 0:
                 base_col_str = f"{base_val:.2f}ms"
        
        # For large scale, we might want to switch baseline to Rust Serial if R is missing/too slow? 
        # But script logic asked for R baseline. Let's stick to R as baseline if present.
        # If R is missing for large scale, we can't compute speedup vs R.

        row_str = f"{display_name:<21} | {base_col_str:^11} |"

        if base_val is None or base_val == 0:
             # Missing baseline
             for _ in candidates:
                 row_str += f" {'-':^13} |" if "Rust (CPU)" in _ else f" {'-':^9} |"
        else:
            # Collect speedups for ranking
            results = []
            
            for cand in candidates:
                if cand == "Rust (CPU)*":
                     serial_data = data.get("Rust (Serial)", {})
                     par_data = data.get("Rust (CPU)", {})
                     s_entry = serial_data.get(name)
                     p_entry = par_data.get(name)
                     
                     s_val = pick_time_value(s_entry)[0] if s_entry else None
                     p_val = pick_time_value(p_entry)[0] if p_entry else None
                     
                     s_speedup_str = "?"
                     p_speedup_str = "?"
                     rank_val = -1
                     
                     # Speedup = R / Rust
                     if s_val and s_val > 0:
                         s_speedup = base_val / s_val
                         s_speedup_str = f"{s_speedup:.1f}" if s_speedup < 10 else f"{s_speedup:.0f}"
                     
                     if p_val and p_val > 0:
                         p_speedup = base_val / p_val
                         rank_val = p_speedup
                         p_speedup_str = f"{p_speedup:.1f}" if p_speedup < 10 else f"{p_speedup:.0f}"
                             
                     disp = "-"
                     if s_speedup_str != "?" or p_speedup_str != "?":
                         disp = f"{s_speedup_str}-{p_speedup_str}x"
                         
                     results.append((cand, rank_val, disp, rank_val))
                     
                else: # Rust (GPU)
                    cand_data = data.get(cand, {})
                    cand_entry = cand_data.get(name)
                    rank_val = -1
                    disp = "-"
                    
                    if cand_entry:
                        c_val, _ = pick_time_value(cand_entry)
                        if c_val and c_val > 0:
                             speedup = base_val / c_val
                             rank_val = speedup
                             disp = f"{speedup:.1f}x"
                    
                    results.append((cand, rank_val, disp, rank_val))

            # Rank
            valid_ranks = sorted([r[3] for r in results if r[3] > 0], reverse=True)
            
            final_cells = []
            for cand, _, disp, r_val in results:
                cell_text = disp
                if r_val > 0 and valid_ranks:
                    if r_val == valid_ranks[0]:
                        cell_text = f"[{disp}]\u00b9"
                    elif len(valid_ranks) > 1 and r_val == valid_ranks[1]:
                         cell_text = f"[{disp}]\u00b2"
                
                final_cells.append(cell_text)

            row_str += f" {final_cells[0]:^13} | {final_cells[1]:^9} |"
            
        print(row_str)

    print("-" * 65)
    print("* Rust (CPU) column shows speedup range: Serial-Parallel vs R (e.g., 12-48x).")
    print("\u00b9 Winner (Fastest implementation)")
    print("\u00b2 Runner-up (Second fastest implementation)")

if __name__ == "__main__":
    main()
