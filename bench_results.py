import subprocess

benches = ['ecs', 'specs', 'recs', 'trex', 'calx_ecs', 'froggy', 'constellation']
bench_targets = ['pos_vel', 'parallel']
bench_names = ['build', 'update']

def parse(out, bench_target, bench_name):
    # indices
    index_target = out.index(bench_target)
    index_bench = out.index(bench_name, index_target)
    index_result = out.index(":", index_bench) + 1
    index_ns = out.index("ns", index_bench)
    index_error = out.index("+/-", index_bench) + 3
    index_end = out.index("\n", index_bench)

    # results and variance
    bench_result = out[index_result:index_ns].strip()
    bench_error = out[index_error:index_end - 1].strip()

    # strip commas and convert to int and then microseconds
    result = int(bench_result.replace(',', '')) / 1000
    error = int(bench_error.replace(',', '')) / 1000

    # re-insert commas
    result = "{:,}".format(result)
    error = "{:,}".format(error)

    return (result, error)

out = subprocess.check_output(["cargo", "bench"], stderr=subprocess.STDOUT)

with open('README.md.tmpl', 'r') as f:
    readme = f.read()

for target in bench_targets:
    for name in bench_names:
        for bench in benches:
            (result, error) = parse(out, target + '_' + bench, 'bench_' + name)
            # microsecond unicode symbol
            ms = u'\u00B5';
            # format like a normal rust benchmark, but in microseconds.
            final = result + " " + ms + "s/iter (+/- " + error + ")"
            readme = readme.replace('{' + target + '_' + name + '_' + bench + '}', final)

with open('README.md', 'w') as f:
    f.write(readme.encode('utf8'))

print(out)
