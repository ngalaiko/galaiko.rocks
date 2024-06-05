-- Point all image links to resized versions.

function Image(el)
	el.src = el.src .. ".800x0@2x.webp"
	el.attributes["width"] = "800px"
	return el
end
