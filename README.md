# rust-hugo-content-maker

A simple thing for generating content for the [Hugo](http://gohugo.io/) static site generator from image files.

# Usage

Pass it:
  - a directory of photos
  - a place to output content
  - the longest side of the "base" image (usually the largest) in px

A folder structure is expected of the following format

```
/<category>/<size>px/<file> 
  
e.g. /outdoors/3200px/dog.jpg
```

The content maker will collect all sizes for a given image name, tagged with the given category (as
a taxonomy of "photos"). The image under the folder with the name matching the third argument will
be read to check the dimensions and these will also be recorded in the content entry, useful for 
scaling etc. or serving images of various dimensions.
