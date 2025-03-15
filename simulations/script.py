import os
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import sys

def read_file(file_path):
    # Read CSV with automatic space trimming
    df = pd.read_csv(file_path, sep=',', skipinitialspace=True)
    return df

def main():
    file_path = sys.argv[1]
    save_file_folder = "imgs"

    # Check if the directory exists
    if not os.path.exists(save_file_folder):
        os.makedirs(save_file_folder)

    # Read the file
    df = read_file(file_path)
    
    # Clean up column names by stripping whitespace
    df.columns = [col.strip() for col in df.columns]
    
    # Set up the plot
    fig, ax = plt.subplots(figsize=(12, 8))
    
    # Set width of bars
    barWidth = 0.2
    
    # Extract the iteration counts and attempt columns
    iterations = df['max_iterations'].tolist()
    attempt_columns = [col for col in df.columns if col != 'max_iterations']
    
    # Create positions for the bars
    r = np.arange(len(attempt_columns))
    positions = []
    
    # Dictionary to store statistics for each iteration count
    stats = {}
    
    # Colors for different iteration counts
    colors = plt.cm.viridis(np.linspace(0, 0.8, len(iterations)))
    
    # Plot bars for each iteration count with different colors
    for i, iter_count in enumerate(iterations):
        # Create positions for this group of bars
        positions.append([x + i * barWidth for x in r])
        
        # Get the attempt counts for this iteration
        values = df.loc[i, attempt_columns].tolist()
        
        # Plot the bars
        ax.bar(positions[i], values, width=barWidth, 
               label=f'{iter_count} iterations',
               color=colors[i],
               alpha=0.8)
        
        # Calculate statistics
        total_attempts = sum(values)
        
        # Modified weighted sum calculation to handle non-numeric column names
        weighted_sum = 0
        for col, count in zip(attempt_columns, values):
            if col not in ['>6', '6+']:  # Skip the failure column
                try:
                    weighted_sum += int(col) * count
                except ValueError:
                    print(f"Warning: Could not convert column '{col}' to integer")
                    
        # Identify failure column and count
        failure_col = '>6' if '>6' in attempt_columns else '6+'
        failure_idx = attempt_columns.index(failure_col) if failure_col in attempt_columns else -1
        failures = values[failure_idx] if failure_idx >= 0 else 0
        
        successes = total_attempts - failures
        
        # Calculate average attempts (excluding failures)
        avg_attempts = weighted_sum / successes if successes > 0 else 0
        
        # Calculate variance - modified to handle non-numeric column names
        variance = 0
        for col, count in zip(attempt_columns, values):
            if col not in ['>6', '6+']:  # Skip the failure column
                try:
                    col_int = int(col)
                    variance += ((col_int - avg_attempts) ** 2) * count
                except ValueError:
                    pass  # Skip non-numeric columns
                    
        variance = variance / successes if successes > 0 else 0
        
        # Calculate success rate
        success_rate = successes / total_attempts if total_attempts > 0 else 0
        
        stats[iter_count] = {
            'avg_attempts': avg_attempts,
            'variance': variance,
            'success_rate': success_rate * 100  # Convert to percentage
        }
    
    # Find the maximum value for y-axis scaling
    max_value = max([max(df.loc[i, attempt_columns]) for i in range(len(iterations))])
    
    # Add better y-axis scaling with nice round numbers
    y_max = max_value * 1.2  # Add 20% padding
    ax.set_ylim(0, y_max)
    
    # Use more tick marks for better readability
    ax.yaxis.set_major_locator(plt.MaxNLocator(10))
    
    # Add labels and title
    ax.set_xlabel('Number of Attempts', fontsize=12, fontweight='bold')
    ax.set_ylabel('Count', fontsize=12, fontweight='bold')
    ax.set_title('Word Solving Performance by Number of Attempts', fontsize=15, fontweight='bold')
    
    # Set x-tick positions and labels
    ax.set_xticks([r + barWidth * (len(iterations)-1)/2 for r in range(len(attempt_columns))])
    ax.set_xticklabels(attempt_columns, fontsize=10, fontweight='bold')
    
    # Add legend
    ax.legend(title='Iterations', fontsize=10)
    
    # Add grid for better readability
    ax.grid(axis='y', linestyle='--', alpha=0.7)
    
    # Add statistics annotations with iteration info and better positioning
    box_colors = ['lightyellow', 'lightblue', 'lightgreen']
    
    for i, iter_count in enumerate(iterations):
        # Position boxes at different heights to avoid overlap
        ax.annotate(
            f"Iterations: {iter_count}\n"
            f"Avg attempts: {stats[iter_count]['avg_attempts']:.2f}\n"
            f"Variance: {stats[iter_count]['variance']:.2f}\n"
            f"Success rate: {stats[iter_count]['success_rate']:.1f}%", 
            xy=(0.75, 0.85 - i*0.12), xycoords='axes fraction',
            fontsize=10, ha='center',
            bbox=dict(boxstyle="round,pad=0.5", facecolor=box_colors[i % len(box_colors)], 
                     alpha=0.8, edgecolor='gray')
        )
    
    plt.tight_layout()
    plt.margins(x=0.05)  # Add margins for better display
    
    plt.savefig(os.path.join(save_file_folder, 'word_solving_performance.png'), dpi=300, bbox_inches='tight')
    #plt.show()  # Uncomment to display the plot

if __name__ == '__main__':
    main()