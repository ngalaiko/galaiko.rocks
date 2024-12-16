-- Point all image links to resized versions.
function Image(el)
	el.src = el.src .. ".800x0@2x.webp"
	el.attributes["width"] = "800px"
	return el
end

local function replace_extension(s, ext)
	return s:gsub("%.%w+$", ext)
end

-- Replace '.cook' and '.md' extensions in local links to '.html'
function Link(el)
	if el.target:match("^%./.*%.cook$") then
		el.target = replace_extension(el.target, ".html")
	elseif el.target:match("^%./.*%.md$") then
		el.target = replace_extension(el.target, ".html")
	end
	return el
end
