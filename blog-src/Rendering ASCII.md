With most exams written I finally found the time to work on some cool projects again, first of which is a Image to ASCII Art converter. 

I first had the idea when checking out [nothings](https://github.com/nothing/stb) single header file libraries, of which `image.h` provides a really simple way for working with all the image formats and does the heavy lifting of working with the different formats for you. 
In my previous project [PNGenius](https://github.com/nailuj05/PNGenius) I used `libpng` itself, which was overkill for the simple functionality I actually needed (and used).

For rendering ascii I had to goals:
- Resemble the source image as closely as possible by calculating the actual lumiance of each char in the font
- Create a simple edge detection algorithm to outline areas of high contrast using `|`, `/`¸ `-`, `\`

To calculate a luminance value for each letter I created a python script which renders every character using `pillow`, calculates the luminance and stores it. Chars that have almost the same luminance are thrown out so there is a mostly even distribution. 

The actual rendering is done in C, where I read the image, convert it to gray scale and then use the gray value to determine the luminance (and thus the char) to be rendered. 

For the edge detection I used a simple 2x2 kernel that scans the image, calculating a vector for each cell that points in the direction of the contrast (the details can be found in the implementation), the kernel also calculates the average luminance of the 4 pixel, it then decides whether to draw an edge or luminance char based on the magnitude of the vector and a threshhold that can be modified for different results. 

![[Pasted image 20250308194318.png]]
*Example showing the 2x2 kernel result and source image*

More details on the kernel and the entire project can be found in its [repository](https://github.com/nailuj05/asciirendering)
