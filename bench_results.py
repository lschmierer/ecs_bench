import subprocess


benches = ['ecs', 'specs', 'recs', 'trex', 'calx_ecs', 'froggy', 'constellation']
bench_targets = ['pos_vel', 'parallel']
bench_names = ['build', 'update']

libraries = ' '.join(benches)
print libraries

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
        target_name = target + "_" + name

        for i, bench in enumerate(benches):
            result = parse(out, target + '_' + bench, 'bench_' + name)
            with open("data/" + target_name, 'w') as dat:
                dat.write("{} {} {} 0.0", bench, i * 15, result)

            readme = readme.replace('{' + target + '_' + name + '_' + bench + '}', result)

        args = "\"bench='{}';data='data/{}';libraries='{}';\""
        args.format(target_name.replace('_', ' '), target_name, libraries);

        subprocess.call(
            ["gnuplot", "-e", args, "bar.plot", ">", target_name + ".svg"],
            stderr=subprocess.STDOUT
        )

with open('README.md', 'w') as f:
    f.write(readme)

print(out)
