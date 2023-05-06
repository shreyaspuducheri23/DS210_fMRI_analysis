# Analyzing Brain fMRI Connectivity Graph Using Graph Theory
This project aims to analyze brain functional MRI (fMRI) connectivity data using graph theory. Specifically, the project uses the USC Multimodal Connectivity Database, which contains preprocessed and organized connectivity matrices that represent the functional connectivity between different regions of the brain during various tasks. The project implements agglomerative clustering based on average neighboring cluster correlations to identify primary functional brain regions.

## Dataset Description
The USC Multimodal Connectivity Database contains brain imaging data from healthy and clinical populations. The fMRI connectivity graph data in this dataset is preprocessed and organized into connectivity matrices, which represent the functional connectivity between different regions of the brain during various tasks. The dataset also includes brain region labels for each vertex and 3-D coordinates for each vertex.

## Graph Representation
The fMRI connectivity data is represented as a weighted graph, where vertices represent different 3x3x3mm voxels of the brain, and edge weights represent the strength of the correlation in functional activity between vertices. For this project, an average correlation matrix was constructed from the correlation matrices of 6 subjects from the USC Multimodal Connectivity Database.

## Running the Code

This Rust project consists of several modules and tests to perform agglomerative clustering on functional magnetic resonance imaging (fMRI) data. The tests are located in the main.rs file, which can be executed to verify that the functions are working as expected. The recommended way to execute this project is to use cargo because it handles dependencies and building the project automatically.

### To run the tests, follow these steps:

1. Install Rust and the Rust package manager, Cargo, on your computer by following the instructions on the official Rust website: https://www.rust-lang.org/tools/install.
2. Clone or download the project from its repository.
3. Navigate to the root directory of the project.
4. In a terminal, run the command cargo test. This command will compile the project and run all the tests in the main.rs file.


### To run the project, follow these steps:

1. Install Rust and the Rust package manager, Cargo: First, you need to install Rust on your machine. You can download it from the official Rust website at https://www.rust-lang.org/tools/install.
2. Clone the repository: Clone the repository to your local machine.
3. Build and run the project: Navigate to the project directory, and run the project using cargo run command. This command will build and compile the project and generate a binary file in the target/debug directory.

## Output
The output of the code includes 4 visualizations of the identified primary functional brain regions. Each of the identified primary functional brain regions are shown in different colors. The following images can be found in `/results/images`

<img src="https://user-images.githubusercontent.com/103334331/236632909-e1dd11b2-e202-46f9-8397-e2c8e2bdae67.png" width="200" /> <img src="https://user-images.githubusercontent.com/103334331/236632910-09ba3b68-76c5-4563-8140-d73363d56ff6.png" width="200" /> <img src="https://user-images.githubusercontent.com/103334331/236632914-ffb66ec5-0337-4666-8b37-b073c9321a7b.png" width="200" /> <img src="https://user-images.githubusercontent.com/103334331/236632919-83381174-d31f-4721-824f-b5c6a8ba2169.png" width="200" />

The output also includes a text file, which can be found at `/results/brain_regions` that lists the identified primary functional brain regions. The 8 identified brain regions are as follows, along with possible interpretations listed alongside each of the brain regions: 

 ### Region 1: Visual  processing and object recognition
> ["Right Lateral Occipital Cortex inferior division"]

 ### Region 2: Vital funcitons, such as breathing, heart rate, conciousness
> ["Brain-Stem", "Brain-Stem", "Cerebellum Vermis IX", "Brain-Stem"]

 ### Region 3: Visual provessing
> ["Left Occipital Pole", "Right Occipital Pole", "Right Occipital Pole", "Left Occipital Pole", "Right Occipital Fusiform Gyrus", "Left Occipital Fusiform Gyrus", "Right Intracalcarine Cortex", "Left Occipital Pole", "Left Lingual Gyrus", "Right Lingual Gyrus"]

 ### Region 4: Language, sensory, and emotional processing and some motor control functions
> ["Right Temporal Pole", "Left Temporal Pole", "Left Planum Polare", "Right Planum Polare", "Right Parietal Operculum Cortex", "Left Parietal Operculum Cortex", "Left Planum Temporale", "Right Supramarginal Gyrus anterior division", "Right Planum Temporale", "Left Central Opercular Cortex", "Left Insular Cortex", "Right Insular Cortex", "Right Temporal Pole", "Right Insular Cortex", "Left Insular Cortex", "Right Pallidum", "Left Pallidum"]

 ### Region 5: Motor control, sensory processing, cognitive processing
> ["Right Crus II", "Left Lateral Occipital Cortex inferior division", "Right Cerebellum Crus II", "Left Cerebellum Crus II", "Right Crus I", "Left Cerebellum Crus I", "Right Parahippocampal Gyrus posterior division", "Left Temporal Occipital Fusiform Cortex", "Brain-Stem", "Left Hippocampus", "Hypothalamus", "Left Parahippocampal Gyrus anterior division", "Right Parahippocampal Gyrus anterior division", "Left Frontal Orbital Cortex", "Left Cerebellum VI", "Right Cerebellum VI", "Brain-Stem", "Cerebellum Vermis VI", "Right Crus I", "Left Cerebellum Crus II", "Right Cerebellum VIIIb", "Left Cerebellum VIIIb", "Left Cerebellum VIIIa", "Right Cerebellum VIIb", "Right Cerebellum VIIb", "Left Cerebellum VIIb", "Left Cerebellum IX", "Right Cerebellum VIIIa", "Right Temporal Occipital Fusiform Cortex", "Right Temporal Fusiform Cortex posterior division", "Left Temporal Fusiform Cortex posterior division", "Left Cerebellum VI", "Left Temporal Pole", "Right Temporal Pole", "Left Temporal Pole", "Right Temporal Pole", "Right Inferior Temporal Gyrus anterior division", "Right Temporal Fusiform Cortex anterior division", "Left Temporal Pole", "Left Temporal Fusiform Cortex anterior division", "Right Inferior Temporal Gyrus posterior division", "Left Inferior Temporal Gyrus posterior division", "Right Inferior Temporal Gyrus posterior division"]

 ### Region 6: Language processing, visual processing, attention
> ["Left Middle Temporal Gyrus posterior division", "Left Middle Temporal Gyrus posterior division", "Left Middle Temporal Gyrus anterior division", "Right Middle Temporal Gyrus anterior division", "Right Middle Temporal Gyrus posterior division", "Right Middle Temporal Gyrus posterior division", "Left Angular Gyrus", "Left Middle Temporal Gyrus temporooccipital part", "Right Angular Gyrus", "Right Middle Temporal Gyrus temporooccipital part", "Left Frontal Orbital Cortex", "Right Frontal Pole", "Left Precuneous Cortex", "Left Cingulate Gyrus posterior division", "Left Precuneous Cortex", "Right Precuneous Cortex", "Right Precuneous Cortex", "Left Lateral Occipital Cortex superior division", "Right Lateral Occipital Cortex superior division"]

 ### Region 7: Decision making, emotional processing, and reward system
> ["Left Frontal Pole", "Left Paracingulate Gyrus", "Right Frontal Pole", "Left Frontal Orbital Cortex", "Right Subcallosal Cortex", "Left Frontal Pole", "Right Frontal Pole", "Right Frontal Pole", "Right Frontal Pole", "Left Frontal Pole", "Right Caudate", "Left Caudate", "Left Thalamus", "Right Thalamus", "Right Thalamus", "Left Caudate", "Right Caudate"]

 ### Region 8: Language processing, motor control, and attention
> ["Right Inferior Frontal Gyrus pars opercularis", "Right Precentral Gyrus", "Right Frontal Pole", "Right Middle Frontal Gyrus", "Right Frontal Pole", "Right Cingulate Gyrus anterior division", "Left Cingulate Gyrus anterior division", "Left Frontal Pole", "Right Frontal Pole", "Right Frontal Pole", "Left Frontal Pole", "Right Lateral Occipital Cortex superior division", "Left Lateral Occipital Cortex superior division", "Left Lateral Occipital Cortex superior division", "Right Lateral Occipital Cortex superior division", "Left Lateral Occipital Cortex superior division", "Right Lateral Occipital Cortex superior division", "Left Superior Parietal Lobule", "Right Superior Parietal Lobule", "Left Angular Gyrus", "Right Angular Gyrus", "Left Inferior Temporal Gyrus posterior division", "Right Lateral Occipital Cortex inferior division", "Left Inferior Temporal Gyrus temporooccipital part", "Right Inferior Temporal Gyrus temporooccipital part", "Left Precentral Gyrus", "Left Postcentral Gyrus", "Right Postcentral Gyrus", "Left Precentral Gyrus", "Right Precentral Gyrus", "Left Superior Frontal Gyrus", "Left Postcentral Gyrus", "Left Superior Parietal Lobule", "Right Superior Parietal Lobule", "Right Precentral Gyrus", "Right Lateral Occipital Cortex superior division", "Left Lateral Occipital Cortex superior division", "Left Superior Frontal Gyrus", "Right Superior Frontal Gyrus", "Left Superior Frontal Gyrus", "Right Superior Frontal Gyrus", "Right Frontal Pole", "Left Frontal Pole", "Right Middle Frontal Gyrus", "Right Middle Frontal Gyrus", "Right Precentral Gyrus", "Left Precentral Gyrus", "Left Lateral Occipital Cortex inferior division", "Right Lateral Occipital Cortex inferior division", "Left Occipital Pole", "Right Occipital Pole", "Left Lateral Occipital Cortex inferior division", "Right Occipital Pole", "Left Inferior Frontal Gyrus pars opercularis", "Left Frontal Pole", "Left Precentral Gyrus", "Left Middle Frontal Gyrus", "Left Middle Frontal Gyrus", "Right Superior Frontal Gyrus", "Left Middle Frontal Gyrus", "Right Frontal Pole", "Left Frontal Pole", "Left Paracingulate Gyrus", "Left Supramarginal Gyrus posterior division", "Right Supramarginal Gyrus posterior division", "Right Frontal Pole", "Left Frontal Pole", "Right Cingulate Gyrus anterior division", "Left Cingulate Gyrus posterior division", "Right Cingulate Gyrus anterior division", "Left Juxtapositional Lobule Cortex", "Left Postcentral Gyrus", "Right Postcentral Gyrus", "Left Postcentral Gyrus", "Right Postcentral Gyrus", "Right Precentral Gyrus", "Left Postcentral Gyrus", "Right Supramarginal Gyrus anterior division", "Left Supramarginal Gyrus anterior division"]


## Conclusion
This project demonstrates the use of graph theory to analyze brain fMRI connectivity data. The agglomerative clustering approach implemented in the code identifies primary functional brain regions based on average neighboring cluster correlations. The results of this analysis provide insight into the functional connectivity of the brain, which can be useful in understanding brain function.
