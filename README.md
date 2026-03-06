# raytracer-rs

## Config File
The program requires an input file containing lines that determine the output image:
```
imsize <w> <h> // dimensions of the image
eye <x> <y> <z> // position of the camera
viewdir <x> <y> <z> // viewing direction
updir <x> <y> <z> // vector indicating the up direction
hfov <i> // horizontal field of view
bkgcolor <r> <g> <b> <eta> // background color
```
For `updir` and `viewdir`, the program automatically normalizes the vectors. 
For `bkgcolor` and `mtlcolor`, all rgb values are clamped between 0 and 1.
`bkgcolor` includes an `eta` parameter that is used for the index of refraction for the world.
Changing this will change the behavior of refraction, as if the camera were placed in a different medium.

Additionally, the following optional inputs can be included:
```
mtlcolor <Odr> <Odg> <Odb> <Osr> <Osg> <Osb> <ka> <kd> <ks> <n> <alpha> <eta>
light <x> <y> <z> <w> <i>
sphere <x> <y> <z> <r>
texture <file.ppm>
vt <u> <v>
```
Defining `mtlcolor` will affect the color of all objects defined afterwards. 
So, to have objects with differing colors, `mtlcolor` can be defined between them to change the objects that follow. 
It takes as parameters the diffuse color, the specular highlight color, the coefficients for the ambient, diffuse, and specular components, and the specular exponent.

Similarly to `mtlcolor`, a texture will apply to all subsequent objects defined afterwards.
It will search for the texture file **relative to the project root directory**. 
As such, a texture found in a texture/ directory must use `texture/file.ppm` to import it.

The `alpha` and `eta` parameters indicate the level of transparency and the index of refraction, respectively.

`vt` will define a texture coordinate that can be used with a triangle.

There are several ways to define triangles:
```
f <v1> <v2> <v3>
f <v1>/<vt1> <v2>/<vt2> <v3>/<vt3>
f <v1>//<vn1> <v2>//<vn2> <v3>//<vn3>
f <v1>/<vt1>/<vn1> <v2>/<vt2>/<vn2> <v3>/<vt3>/<vn3>
```
Vertices and vertex normals can be defined using `v` and `vn`, respectively:
```
v <x> <y> <z>
vn <nx> <ny> <nz>
```
Each parameter to `f` will be an index in either the vertex array, the vertex normal array, or the texture coordinate array.
Each array is 1-indexed, and the order of each element is based on when it appears in the input file.

Both the texture coordinates and vertex normals are optional. 
Including texture coordinates will give the triangle a texture, and including vertex normals will allow for smooth shading.
