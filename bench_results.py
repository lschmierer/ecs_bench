import subprocess


benches = ['ecs', 'specs', 'recs', 'trex', 'calx_ecs', 'froggy', 'constellation']
bench_targets = ['pos_vel', 'parallel']
bench_names = ['build', 'update']

# gets all the benchmarks
def benchmarks(targets, names):
  list = []
  for target in bench_targets:
    for name in bench_names:
      list.append(target + "_" + name)
  return list

# turns a dictionary into a 2d array
def data_table(dataset, rows, columns):
  table = [["title"] + columns]
  print "table: ", table
  
  for (i, row) in enumerate(rows):
    table.append([row])
    for (j, column) in enumerate(columns):
      table[i + 1].append(dataset[row][column])
  return table
  
# formats the 2d array into a multi-line string
def pretty_table(table):
  result = ""
  for (j, row) in enumerate(table):
    for (i, element) in enumerate(row):
      if i == 0 or j == 0:
        result += " " + element
      else:
        result += " " + str(element[0]) + " " + str(element[1])
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

# initialize library dictionaries
for bench in benches:
    dataset[bench] = {}

for target in bench_targets:
    for name in bench_names:
        target_name = target + "_" + name
        for i, bench in enumerate(benches):
            (result, error) = parse(out, target + '_' + bench, 'bench_' + name)

            # microsecond unicode symbol
            ms = u'\u00B5';

            # format like a normal rust benchmark, but in microseconds.
            final = commas(result) + " " + ms + "s/iter (+/- " + commas(error) + ")"
            readme = readme.replace('{' + target + '_' + name + '_' + bench + '}', final)
            dataset[bench][target + "_" + name] = (result, error)

data = data_table(dataset, benches, benchmarks(bench_targets, bench_names))
formatted = pretty_table(data)
with open("./graph/table.dat", 'w') as dat:
    dat.write(formatted)
    
# commands to send to `gnuplot`
args = "gnuplot -e \"data=\'./graph/table.dat\';\" ./graph/graph.script > ./graph/all.png"
subprocess.call(args, stderr=subprocess.STDOUT, shell=True)

with open('README.md', 'w') as f:
    f.write(readme.encode('utf8'))

print(out)
