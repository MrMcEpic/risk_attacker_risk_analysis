import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

def read_and_pivot_data(file_path, header_names):
	"""Reads a CSV file and pivots the data for a heatmap."""
	try:
		data = pd.read_csv(file_path, header=None, names=header_names)
		data_pivot = data.pivot(index=header_names[0], columns=header_names[1], values=header_names[2])
		return data_pivot
	except FileNotFoundError:
		print(f"File {file_path} not found.")
		return None

def create_heatmap(data_pivot, title, ax, annot_kws_size, x_label, y_label):
	"""Creates a heatmap from pivoted data."""
	heatmap = sns.heatmap(data_pivot, cmap='RdYlGn', ax=ax, annot=True, fmt=".1f", annot_kws={"size": annot_kws_size})
	heatmap.collections[0].colorbar.set_label('Percentage that the Attacker Wins', rotation=270, labelpad=20)
	ax.set_title(title)
	ax.set_xlabel(x_label)
	ax.set_ylabel(y_label)

def main():
	# Read and pivot the data
	simulation_data_pivot = read_and_pivot_data('data/simulation_data.csv', ['Attacker Dice', 'Defender Dice', 'Win Percentage'])
	battle_simulation_data_pivot = read_and_pivot_data('data/battle_simulation_data.csv', ['Attacker Soldiers', 'Defender Soldiers', 'Win Percentage'])

	# Create the subplots
	fig, axs = plt.subplots(ncols=2, figsize=(20, 8))

	# Create the heatmaps
	create_heatmap(simulation_data_pivot, 'Dice Roll Simulation Heatmap', axs[0], 20, 'Defender Dice', 'Attacker Dice')
	create_heatmap(battle_simulation_data_pivot, 'Battle Simulation Heatmap', axs[1], 5, 'Defender Soldiers', 'Attacker Soldiers')

	plt.show()

if __name__ == "__main__":
	main()