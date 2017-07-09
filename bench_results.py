import subprocess
benches = ['ecs', 'specs', 'trex', 'calx_ecs', 'froggy', 'constellation']
bench_targets = ['pos_vel', 'parallel']
bench_names = ['build', 'update']

# turns a dictionary into a 2d array
def table_format(dataset, targets, types, libraries, title):
    table = [[title]]

    for target in targets:
        for type in types:
            table[0].append(target + "_" + type)

    for (i, target) in enumerate(targets):
        for (j, library) in enumerate(libraries):
            if i == 0:
                table.append([])
                table[j + 1].append(library)
            for type in types:
                table[j + 1] += dataset[type][target][library]
    return table 

# formats the 2d array into a multi-line string
def pretty_table(table):
    result = ""
    for (j, row) in enumerate(table):
        for (i, element) in enumerate(row):
            if i == 0 or j == 0:
                result += " " + element
            else:
                result += " " + str(element)
        result += "\n"
    return result

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

dataset = {}
for target in bench_targets:
    for name in bench_names:
        if not name in dataset:
            dataset[name] = {}
        if not target in dataset[name]:
            dataset[name][target] = {}

        for i, bench in enumerate(benches):
            (result, error) = parse(out, target + '_' + bench, 'bench_' + name)

            # microsecond unicode symbol
            ms = u'\u00B5';

            # format like a normal rust benchmark, but in microseconds.
            final = commas(result) + " " + ms + "s/iter (+/- " + commas(error) + ")"
            readme = readme.replace('{' + target + '_' + name + '_' + bench + '}', final)
            dataset[name][target][bench] = (result, error)

def graph(dataset, targets, names, benches, title):
    data = table_format(dataset, targets, names, benches, title)
    formatted = pretty_table(data)
    with open("./graph/" + title + ".dat", 'w') as dat:
        dat.write(formatted)

    # commands to send to `gnuplot`
    args = "gnuplot -e \"data=\'./graph/{0}.dat';title=\'{0}\';\" ./graph/graph.script > ./graph/{0}.png"
    subprocess.call(args.format(title, len(targets) * len(names)), stderr=subprocess.STDOUT, shell=True)

graph(dataset, bench_targets, bench_names, benches, "all")
graph(dataset, bench_targets, ["update"], benches, "update")
graph(dataset, bench_targets, ["build"], benches, "build")

with open('README.md', 'w') as f:
    f.write(readme.encode('utf8'))

print(out)
