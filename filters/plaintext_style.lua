-- Pandoc Lua filter to convert Markdown to a single <pre> HTML tag
-- with 72-character text wrapping and limited tag preservation

-- Add HTML escape function
local function escape_html(text)
	return text:gsub("&", "&amp;"):gsub("<", "&lt;"):gsub(">", "&gt;"):gsub('"', "&quot;"):gsub("'", "&#39;")
end

-- Custom map function to replace pandoc.utils.map
local function map(array, func)
	local new_array = {}
	for i, v in ipairs(array) do
		new_array[i] = func(v)
	end
	return new_array
end

-- Function to calculate visible character count for links
local function visible_char_count(text)
	return #text:gsub("<[^>]+>", "")
end

-- Function to wrap text to a specified width
local function wrap_text(text, width)
	local wrapped = {}
	local current_line = ""

	for word in text:gmatch("%S+") do
		local test_line = current_line ~= "" and current_line .. " " .. word or word

		if visible_char_count(test_line) > width then
			table.insert(wrapped, current_line)
			current_line = word
		else
			current_line = test_line
		end
	end

	if current_line ~= "" then
		table.insert(wrapped, current_line)
	end

	return table.concat(wrapped, "\n")
end

-- Function to convert inline elements to plain text with preserved tags
local function inline_to_plaintext(inline)
	if inline.t == "Str" then
		return inline.text
	elseif inline.t == "Space" then
		return " "
	elseif inline.t == "SoftBreak" or inline.t == "LineBreak" then
		return "\n"
	elseif inline.t == "Link" then
		local content = pandoc.utils.stringify(inline.content)
		return string.format("<a href='%s'>%s</a>", inline.target, content)
	elseif inline.t == "Image" then
		local alt_text = pandoc.utils.stringify(inline.caption)
		return string.format("<img src='%s' alt='%s'>", inline.src, alt_text)
	else
		return ""
	end
end

-- Function to process block elements
local function block_to_plaintext(block)
	if block.t == "Para" or block.t == "Plain" then
		local text = table.concat(map(block.content, inline_to_plaintext))
		return wrap_text(text, 72)
	elseif block.t == "Figure" then
		local image = block.content[1] -- The first element is usually the image
		local caption = pandoc.utils.stringify(block.caption)
		if image and image.t == "Plain" and image.content[1].t == "Image" then
			local img = image.content[1]
			local alt_text = pandoc.utils.stringify(img.caption)
			return string.format(
				"<figure><img src='%s' alt='%s'><figcaption>%s</figcaption></figure>",
				img.src,
				alt_text,
				caption
			)
		end
	elseif block.t == "Header" then
		local level = string.rep("#", block.level)
		local text = table.concat(map(block.content, inline_to_plaintext))
		return wrap_text(level .. " " .. text, 72)
	elseif block.t == "BulletList" then
		local items = {}
		for _, item in ipairs(block.content) do
			-- Join the item contents with newlines between blocks
			local item_text = table.concat(map(item, block_to_plaintext), "\n")
			table.insert(items, "- " .. item_text:gsub("\n", "\n  ")) -- Indent continued lines
		end
		return table.concat(items, "\n")
	elseif block.t == "OrderedList" then
		local items = {}
		for i, item in ipairs(block.content) do
			-- Join the item contents with newlines between blocks
			local item_text = table.concat(map(item, block_to_plaintext), "\n")
			-- Calculate padding for alignment of continued lines
			local number_width = #tostring(i) + 2 -- accounts for number and ". "
			local padding = string.rep(" ", number_width)
			-- Add the numbered item and indent continued lines
			table.insert(items, string.format("%d. %s", i, item_text:gsub("\n", "\n" .. padding)))
		end
		return table.concat(items, "\n")
	elseif block.t == "BlockQuote" then
		local text = table.concat(map(block.content, block_to_plaintext))
		return wrap_text("> " .. text, 72)
	elseif block.t == "HorizontalRule" then
		return string.rep("-", 72)
	elseif block.t == "Table" then
		return wrap_text("[Table content omitted for plaintext format]", 72)
	else
		return ""
	end
end

-- Function to center text within 72 characters
local function center_text(text, width)
	local padding_length = math.floor((width - #text) / 2)
	return string.rep(" ", padding_length) .. text
end

-- Main Pandoc filter
function Pandoc(doc)
	local all_text = {}

	local title = doc.meta.title and pandoc.utils.stringify(doc.meta.title) or ""
	if title ~= "" then
		table.insert(all_text, center_text(title, 72))
	end

	for _, block in ipairs(doc.blocks) do
		local block_text = block_to_plaintext(block)
		if block_text ~= "" then
			table.insert(all_text, block_text)
		end
	end
	local output = table.concat(all_text, "\n\n")
	return pandoc.Pandoc({ pandoc.RawBlock("html", output) }, doc.meta)
end
