import os
import re
import pandas as pd


from cutecharts.charts import Line

def main():
    directory = "../out"

    pattern = re.compile(r'benchmark-(.*?)-(.*?)\.csv')

    sizes = None

    throughputs = [] # format is (prefix, suffix, throughput)
    latencies = [] # format is (prefix, suffix, latency)

    for filename in os.listdir(directory):
        match = pattern.match(filename)
        if match:
            prefix, suffix = match.groups()
            file_path = os.path.join(directory, filename)

            df = pd.read_csv(file_path)

            sizes = df['Size'].tolist() if sizes is None else sizes
            latencies_ = df['Latency'].tolist()
            throughputs_ = df['Throughput'].tolist()

            latencies.append((prefix, suffix, latencies_))
            throughputs.append((prefix, suffix, throughputs_))

    sizes = [] if sizes is None else sizes

    chart = Line("Latencies")
    chart.set_options(
        labels=sizes,
        x_label="Size of the Payload",
        y_label="Latency (µs)",
        legend_pos="upLeft",
    )
    for (prefix, suffix, latencies_) in latencies:
        prefix = "wrapped" if prefix == "" else prefix

        chart.add_series(f"{prefix} {suffix}", latencies_)

    chart.render("../bench/latencies.html")
    file = open("../bench/latencies.html", "r")
    content = file.read()
    file.close()

    content_latencies = content

    chart = Line("Throughputs")
    chart.set_options(
        labels=sizes,
        x_label="Size of the Payload",
        y_label="Throughput frequency (s⁻¹)",
    )

    for (prefix, suffix, throughputs_) in throughputs:
        prefix = "wrapped" if prefix == "" else prefix

        chart.add_series(f"{prefix} {suffix}", throughputs_)

    chart.render("../bench/throughputs.html")
    file = open("../bench/throughputs.html", "r")
    content = file.read()
    file.close()

    content_throughputs = content

    combined = content_latencies + content_throughputs

    file = open("../docs/src/flarrow-benchmark.md", "w")
    file.write(combined)
    file.close()

if __name__ == "__main__":
    main()
