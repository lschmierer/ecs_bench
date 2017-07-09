import subprocess


benches = ['ecs', 'specs', 'trex', 'calx_ecs', 'froggy', 'constellation']
bench_targets = ['pos_vel', 'parallel']
bench_names = ['build', 'update']
libraries = ' '.join(benches)

def commas(num):
    return "{:,}".format(num)

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

    return (result, error)

out = subprocess.check_output(["cargo", "bench"], stderr=subprocess.STDOUT)

with open('README.md.tmpl', 'r') as f:
    readme = f.read()

for target in bench_targets:
    for name in bench_names:
        target_name = target + "_" + name
        data = ""
        for i, bench in enumerate(benches):
            (result, error) = parse(out, target + '_' + bench, 'bench_' + name)

            # microsecond unicode symbol
            ms = u'\u00B5';

            # format like a normal rust benchmark, but in microseconds.
            final = commas(result) + " " + ms + "s/iter (+/- " + commas(error) + ")"
            readme = readme.replace('{' + target + '_' + name + '_' + bench + '}', final)

            data = data + "{} {} {} {}\n".format(bench, i * 10 + 5, result, error)

        with open("data/" + target_name + ".dat", 'w') as dat:
            dat.write(data)

        # commands to send to `gnuplot`
        commands = '''"bench='{}';data='./data/{}.dat';libraries='{}';"'''
        commands = commands.format(target_name.replace('_', ' '), target_name, libraries);
        args = "gnuplot -e " + commands + " plot.dem > ./graphs/" + target_name + ".svg"
        subprocess.call(args, stderr=subprocess.STDOUT, shell=True)

with open('README.md', 'w') as f:
    f.write(readme.encode('utf8'))

print(out)
