import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

# Read the CSV files
simulation_data = pd.read_csv('data/simulation_data.csv', header=None, names=['Attacker Dice', 'Defender Dice', 'Win Percentage'])
battle_simulation_data = pd.read_csv('data/battle_simulation_data.csv', header=None, names=['Attacker Soldiers', 'Defender Soldiers', 'Win Percentage'])

# Pivot the data for the heatmaps
simulation_data_pivot = simulation_data.pivot(index='Attacker Dice', columns='Defender Dice', values='Win Percentage')
battle_simulation_data_pivot = battle_simulation_data.pivot(index='Attacker Soldiers', columns='Defender Soldiers', values='Win Percentage')

# Create the subplots
fig, axs = plt.subplots(ncols=2, figsize=(20, 8))

# Create the heatmaps
heatmap1 = sns.heatmap(simulation_data_pivot, cmap='RdYlGn', ax=axs[0], annot=True, fmt=".1f", annot_kws={"size": 20})
heatmap1.collections[0].colorbar.set_label('Percentage that the Attacker Wins the Dice Roll', rotation=270, labelpad=20)
axs[0].set_title('Dice Roll Simulation Heatmap')

heatmap2 = sns.heatmap(battle_simulation_data_pivot, cmap='RdYlGn', ax=axs[1], annot=True, fmt=".1f", annot_kws={"size": 5})
heatmap2.collections[0].colorbar.set_label('Percentage that the Attacker Wins the Battle', rotation=270, labelpad=20)
axs[1].set_title('Battle Simulation Heatmap')
plt.show()