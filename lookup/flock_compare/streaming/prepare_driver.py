from pathlib import Path
p=Path(__file__).resolve().parent/'server_driver.py'
s=p.read_text(encoding='utf-8').replace("work=source/'optimized'", "work=source/'streaming'")
s=s.replace("subprocess.check_output(['lscpu','-J'])", "subprocess.check_output(['lscpu','-J'],env=env)")
start=s.index("case('smoke-n20-48t'")
end=s.index("inventory['memory_final']",start)
s=s[:start]+'''case('smoke-n20-48t',20,48,'previous,release,optimized,previous_full,full,flock',1)
case('ablation-n26-48t',26,48,'previous,release,optimized,previous_full,full,flock',6)
for threads in (24,48):
    case(f'main-n30-{threads}t',30,threads,'previous,previous_full,optimized,full,flock',5)
    for system in ('previous','previous_full','optimized','full','flock'):
        case(f'cold-rss-n30-{threads}t-{system}',30,threads,system,1,0,1)
'''+s[end:]
p.write_text(s,encoding='utf-8')
