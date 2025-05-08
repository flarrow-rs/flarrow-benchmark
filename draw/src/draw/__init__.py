import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import os
import glob

# Enable XKCD style
plt.xkcd()

# Function to read all CSV files in the out/ folder
def read_all_csv_files():
    # Get all CSV files in the directory
    csv_files = glob.glob(os.path.join("../out", '*.csv'))

    if not csv_files:
        print(f"No CSV files found in ../out folder")
        return None

    # Dictionary to store dataframes
    dataframes = {}

    # Read each CSV file
    for file in csv_files:
        filename = os.path.basename(file)
        try:
            df = pd.read_csv(file)
            dataframes[filename] = df
            print(f"File loaded: {filename}")
        except Exception as e:
            print(f"Error reading {filename}: {e}")

    return dataframes

# Function to plot benchmark results
def plot_benchmark_results(dataframes):
    if not dataframes:
        return

    fig_latency = plt.figure()
    ax_latency = fig_latency.add_subplot(111)
    fig_throughput = plt.figure()
    ax_throughput = fig_throughput.add_subplot(111)

    # Colors for different files
    colors = ['blue', 'red', 'green', 'purple', 'orange', 'brown', 'pink', 'gray', 'olive', 'cyan']

    # For each CSV file
    for i, (filename, df) in enumerate(dataframes.items()):
        color = colors[i % len(colors)]
        label_base = filename.replace('.csv', '')

        # 1. Latency vs Size Plot
        ax_latency.semilogx(df['size'], df['latency_us'], 'o-', color=color, label=label_base, linewidth=2)

        # 2. Throughput (Gbps) vs Size Plot
        ax_throughput.loglog(df['size'], df['throughput_gbps'] * 1000, 'o-', color=color, label=label_base, linewidth=2)

    # Configure titles and labels
    ax_latency.set_title("Latency vs Size (Log Scale)")
    ax_latency.set_xlabel("Size (bytes) - Log Scale")
    ax_latency.set_ylabel("Latency (μs)")

    ax_throughput.set_title("Throughput (MBps) vs Size (Log Scale)")
    ax_throughput.set_xlabel("Size (bytes) - Log Scale")
    ax_throughput.set_ylabel("Throughput (MBps)")

    # # Create a single legend outside the plots
    fig_legend = plt.figure()

    handles, labels = ax_latency.get_legend_handles_labels()
    fig_legend.legend(handles, labels, loc='center',
                      fancybox=True, shadow=True)

    fig_legend.savefig('../bench/benchmark_legend.svg', format='svg')

    fig_legend.savefig('../bench/benchmark_legend.png', format='png', dpi=300)

    if ax_latency.get_legend():
        ax_latency.get_legend().remove()

    if ax_throughput.get_legend():
        ax_throughput.get_legend().remove()

    # Adjust layout
    fig_latency.tight_layout()
    fig_throughput.tight_layout()

    # Save as SVG
    fig_latency.savefig('../bench/benchmark_latency.svg', format='svg')
    fig_latency.savefig('../bench/benchmark_latency.png', format='png', dpi=300)

    # Also save as PNG for quick viewing
    fig_throughput.savefig('../bench/benchmark_throughput.svg', format='svg')
    fig_throughput.savefig('../bench/benchmark_throughput.png', format='png', dpi=300)

# Main execution
def main():
    # Read all CSV files
    dataframes = read_all_csv_files()

    # Plot graphs if files were found
    if dataframes:
        plot_benchmark_results(dataframes)
    else:
        print("No graphs were created because no CSV files were found.")
