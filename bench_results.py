import subprocess


benches = ['ecs', 'specs', 'recs', 'trex']
bench_targets = ['pos_vel', 'parallel']
bench_names = ['build', 'update']

def parse(out, bench_target, bench_name):
    index_target = out.index(bench_target)
    index_bench = out.index(bench_name, index_target)
    index_result = out.index(":", index_bench) + 1
    index_end = out.index("\n", index_bench)
    result = out[index_result:index_end].strip()
    return result

out = subprocess.check_output(["cargo", "bench"], stderr=subprocess.STDOUT)

with open('README.md.tmpl', 'r') as f:
    readme = f.read()

for target in bench_targets:
    for name in bench_names:
        for bench in benches:
            result = parse(out, target + '_' + bench, 'bench_' + name)
            readme = readme.replace('{' + target + '_' + name + '_' + bench + '}', result)

with open('README.md', 'w') as f:
    f.write(readme)

print(out)
