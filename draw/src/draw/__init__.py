# import os
# import re
# import pandas as pd

# from cutecharts.charts import Line

# def main():
#     directory = "../out"

#     pattern = re.compile(r'benchmark-(.*?)-(.*?)\.csv')

#     sizes = None

#     throughputs = [] # format is (prefix, suffix, throughput)
#     latencies = [] # format is (prefix, suffix, latency)

#     for filename in os.listdir(directory):
#         match = pattern.match(filename)
#         if match:
#             prefix, suffix = match.groups()
#             file_path = os.path.join(directory, filename)

#             df = pd.read_csv(file_path)

#             sizes = df['Size'].tolist() if sizes is None else sizes
#             latencies_ = df['Latency'].tolist()
#             throughputs_ = df['Throughput'].tolist()

#             latencies.append((prefix, suffix, latencies_))
#             throughputs.append((prefix, suffix, throughputs_))

#     sizes = [] if sizes is None else sizes

#     chart = Line("Latencies")
#     chart.set_options(
#         labels=sizes,
#         x_label="Size of the Payload",
#         y_label="Latency (µs)",
#         legend_pos="upRight"
#     )
#     for (prefix, suffix, latencies_) in latencies:
#         prefix = "wrapped" if prefix == "" else prefix

#         chart.add_series(f"{prefix} {suffix}", latencies_)

#     chart.render("../bench/latencies.html")

#     chart = Line("Throughputs")
#     chart.set_options(
#         labels=sizes,
#         x_label="Size of the Payload",
#         y_label="Throughput frequency (s⁻¹)",
#     )

#     for (prefix, suffix, throughputs_) in throughputs:
#         prefix = "wrapped" if prefix == "" else prefix

#         chart.add_series(f"{prefix} {suffix}", throughputs_)

#     chart.render("../bench/throughputs.html")


# if __name__ == "__main__":
#     main()

from cutecharts.charts import Line
from cutecharts.components import Page
from cutecharts.faker import Faker


def line_base() -> Line:
    chart = Line("Line-基本示例")
    chart.set_options(labels=Faker.choose(), x_label="I'm xlabel", y_label="I'm ylabel")
    chart.add_series("series-A", Faker.values())
    chart.add_series("series-B", Faker.values())
    return chart


line_base().render()


def line_legend():
    chart = Line("Line-Legend 位置")
    chart.set_options(labels=Faker.choose(), legend_pos="upRight")
    chart.add_series("series-A", Faker.values())
    chart.add_series("series-B", Faker.values())
    return chart


def line_tickcount_colors():
    chart = Line("Line-调整颜色")
    chart.set_options(labels=Faker.choose(), colors=Faker.colors, y_tick_count=8)
    chart.add_series("series-A", Faker.values())
    chart.add_series("series-B", Faker.values())
    return chart


page = Page()
page.add(line_base(), line_legend(), line_tickcount_colors())
page.render()
