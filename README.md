# rust-app
learning rust v1

to learn rust do the following app:
* desktop(runnable on debian)
* load pdf with the cross-stich schema, parse it and show
* allow highlight defined color
* do zoom
* save progress (optional)

Load schema. Data is stored initially in the pdf (required - read / load image / parse schema from multiple pages(images) into array of colors).
* attempt#1 - pdf-rs. 
    -  Result - seems that not working well with images. at all.
* attempt#2 - image. Load image directly and do simple parsing (without pdf to store them) 
    - Works. Slow. Some sort of database to store data is required. 
       - attempt#1 - sqlite (as in the cookbook example)
 