-- Replace '.cook' and '.md' extensions in local links to '.html'

local function replace_extension(s, ext)
	return s:gsub("%.%w+$", ext)
end

function Link(el)
	if el.target:match("^%./.*%.cook$") then
		el.target = replace_extension(el.target, ".html")
	elseif el.target:match("^%./.*%.md$") then
		el.target = replace_extension(el.target, ".html")
	end
	return el
end
