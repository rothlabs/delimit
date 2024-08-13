# Mecha
Mecha (mechatronics) is a geometry processing crate. There are currently three nodes for importing STL files, validating the mesh, and exporting OBJ files with Go/No-Go material to be viewed in software like Blender. See ```mesh/monkey_with_hole.jpg``` to see what the results look like.

## Documentation 
Run ```cargo doc -p mecha --open``` to view the documentation.

## Testing
First, you must changed the ```mesh/configure.rs``` so the data paths match your system. Replace ```/home/julian/delimit/``` with your absolute path for both ```TEST_DATA_PATH``` and ```TEST_OUT_PATH```.
Run ```cargo test``` within the mecha crate to process the test models located in mesh/test_data. Then open the models in Blender to view the results.

## How it Works
A triangle is “Go” if it touches three other triangles that have the same winding and it does not intersect other triangles. Any triangle that does not meet those conditions is “No-go.”
Triangles are created as parametric shapes with a plotting method. This way the intersection logic can be separate from the shape logic.
Meshes are handled as interweaved attribute arrays so they are ready for upload to the GPU. It also makes it easier to plug and play nodes if meshes are always handled this way.

## Improvement
The algorithm is horribly slow because it compares every triangle pair. This can be improved by sorting the triangles into local locations (spatial hashmap, BVH). Further improvement would involve creating a low level array graph with nodes that utilize the GPU. The array graph logic would not inherently involve mesh-validation logic. The higher mesh-validation graph would simply construct the lower array graph. 

