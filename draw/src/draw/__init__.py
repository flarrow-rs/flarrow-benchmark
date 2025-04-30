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

    # Create a figure with two subplots
    fig, axs = plt.subplots(1, 2, figsize=(15, 7))  # Increased height a bit for legend

    # Colors for different files
    colors = ['blue', 'red', 'green', 'purple', 'orange', 'brown', 'pink', 'gray', 'olive', 'cyan']

    # For each CSV file
    for i, (filename, df) in enumerate(dataframes.items()):
        color = colors[i % len(colors)]
        label_base = filename.replace('.csv', '')

        # 1. Latency vs Size Plot
        axs[0].semilogx(df['size'], df['latency_us'], 'o-', color=color, label=label_base, linewidth=2)

        # 2. Throughput (Gbps) vs Size Plot
        axs[1].semilogx(df['size'], df['throughput_gbps'], 'o-', color=color, label=label_base, linewidth=2)

    # Configure titles and labels
    axs[0].set_title("Latency vs Size (Log Scale)")
    axs[0].set_xlabel("Size (bytes) - Log Scale")
    axs[0].set_ylabel("Latency (μs)")

    axs[1].set_title("Throughput (Gbps) vs Size (Log Scale)")
    axs[1].set_xlabel("Size (bytes) - Log Scale")
    axs[1].set_ylabel("Throughput (Gbps)")

    # Set grid for logarithmic scale (looks better in log scale)
    for ax in axs:
        ax.grid(True, which="both", linestyle='-', alpha=0.3)

    # # Create a single legend outside the plots
    fig_legend = plt.figure()

    handles, labels = axs[0].get_legend_handles_labels()
    fig_legend.legend(handles, labels, loc='center',
                      fancybox=True, shadow=True)

    fig_legend.savefig('../bench/benchmark_legend.svg', format='svg')

    fig_legend.savefig('../bench/benchmark_legend.png', format='png', dpi=300)

    # Remove individual legends
    for ax in axs:
        if ax.get_legend():
            ax.get_legend().remove()

    # Adjust layout
    fig.tight_layout()

    # Save as SVG
    fig.savefig('../bench/benchmark_results.svg', format='svg')

    # Also save as PNG for quick viewing
    fig.savefig('../bench/benchmark_results.png', format='png', dpi=300)

# Main execution
def main():
    # Read all CSV files
    dataframes = read_all_csv_files()

    # Plot graphs if files were found
    if dataframes:
        plot_benchmark_results(dataframes)
    else:
        print("No graphs were created because no CSV files were found.")
