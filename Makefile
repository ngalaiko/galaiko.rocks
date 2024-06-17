# Directories
SRC_DIR := assets
BUILD_DIR := build

# Binaries
PANDOC_BIN := pandoc
IMAGEMAGICK_BIN := magick
JQ_BIN := jq
YQ_BIN := yq
J2_BIN := j2 --customize ./j2_customize.py
COOK_BIN := cook

# Find input files
INPUT_FILES := $(shell find $(SRC_DIR) -type f)

# Define specific file types
INPUT_MD_FILES := $(filter %.md, $(INPUT_FILES))
INPUT_POST_FILES := $(filter $(SRC_DIR)/posts/%.md, $(INPUT_FILES))
INPUT_POST_IMAGE_FILES := $(filter $(SRC_DIR)/posts/%.jpg $(SRC_DIR)/posts/%.jpeg $(SRC_DIR)/posts/%.png, $(INPUT_FILES))
INPUT_MOVIE_FILES := $(filter $(SRC_DIR)/movies/%.json, $(INPUT_FILES))
INPUT_MOVIE_IMAGE_FILES := $(filter $(SRC_DIR)/movies/%.jpg, $(INPUT_FILES))
INPUT_RECORD_FILES := $(filter $(SRC_DIR)/records/%.json, $(INPUT_FILES))
INPUT_RECORD_IMAGE_FILES := $(filter $(SRC_DIR)/records/%.jpeg, $(INPUT_FILES))
INPUT_COCKTAIL_FILES := $(filter $(SRC_DIR)/cocktails/%.cook, $(INPUT_FILES))
INPUT_COCKTAIL_IMAGE_FILES := $(filter $(SRC_DIR)/cocktails/%.jpeg, $(INPUT_FILES))
INPUT_PLACE_FILES := $(filter $(SRC_DIR)/places/%.json, $(INPUT_FILES))

# Other input files
INPUT_OTHER_FILES := $(filter-out $(INPUT_MD_FILES) $(INPUT_POST_FILES) $(INPUT_POST_IMAGE_FILES) $(INPUT_MOVIE_FILES) $(INPUT_MOVIE_IMAGE_FILES) $(INPUT_RECORD_FILES) $(INPUT_RECORD_IMAGE_FILES) $(INPUT_COCKTAIL_FILES) $(INPUT_COCKTAIL_IMAGE_FILES) $(INPUT_PLACE_FILES), $(INPUT_FILES))

# Output file definitions
OUTPUT_MD_FILES := $(patsubst $(SRC_DIR)/%.md,$(BUILD_DIR)/%.html,$(INPUT_MD_FILES))
OUTPUT_IMAGE_FILES := $(patsubst $(SRC_DIR)/%.jpg,$(BUILD_DIR)/%.jpg.800x0@2x.webp,$(filter %.jpg, $(INPUT_POST_IMAGE_FILES)))
OUTPUT_IMAGE_FILES += $(patsubst $(SRC_DIR)/%.jpeg,$(BUILD_DIR)/%.jpeg.800x0@2x.webp,$(filter %.jpeg, $(INPUT_POST_IMAGE_FILES)))
OUTPUT_IMAGE_FILES += $(patsubst $(SRC_DIR)/%.png,$(BUILD_DIR)/%.png.800x0@2x.webp,$(filter %.png, $(INPUT_POST_IMAGE_FILES)))

OUTPUT_MOVIE_IMAGE_FILES := $(patsubst $(SRC_DIR)/%.jpg,$(BUILD_DIR)/%.jpg.200x0@2x.webp,$(INPUT_MOVIE_IMAGE_FILES))
OUTPUT_RECORD_IMAGE_FILES := $(patsubst $(SRC_DIR)/%.jpeg,$(BUILD_DIR)/%.jpeg.200x0@2x.webp,$(INPUT_RECORD_IMAGE_FILES))

OUTPUT_COCKTAIL_FILES := $(patsubst $(SRC_DIR)/%.cook,$(BUILD_DIR)/%.html,$(INPUT_COCKTAIL_FILES))
OUTPUT_COCKTAIL_IMAGE_FILES := $(patsubst $(SRC_DIR)/%.jpeg,$(BUILD_DIR)/%.jpeg.200x0@2x.webp,$(INPUT_COCKTAIL_IMAGE_FILES))
OUTPUT_COCKTAIL_IMAGE_FILES += $(patsubst $(SRC_DIR)/%.jpeg,$(BUILD_DIR)/%.jpeg.800x0@2x.webp,$(INPUT_COCKTAIL_IMAGE_FILES))

OUTPUT_PLACE_FILES := $(patsubst $(SRC_DIR)/%.json,$(BUILD_DIR)/%.html,$(INPUT_PLACE_FILES))
OUTPUT_OTHER_FILES := $(patsubst $(SRC_DIR)/%,$(BUILD_DIR)/%,$(INPUT_OTHER_FILES))

# Combine all outputs
OUTPUT := $(OUTPUT_MD_FILES) $(OUTPUT_COCKTAIL_FILES) $(BUILD_DIR)/cocktails/index.html $(OUTPUT_COCKTAIL_IMAGE_FILES)
OUTPUT += $(BUILD_DIR)/records/index.html $(BUILD_DIR)/movies/index.html $(OUTPUT_MOVIE_IMAGE_FILES)
OUTPUT += $(OUTPUT_RECORD_IMAGE_FILES) $(BUILD_DIR)/posts/index.html $(BUILD_DIR)/posts/index.atom $(BUILD_DIR)/posts/index.xml
OUTPUT += $(OUTPUT_IMAGE_FILES) $(BUILD_DIR)/places/index.html $(OUTPUT_OTHER_FILES)

# Macros
templ = templates/_layout.html.jinja j2_customize.py templates/$1

.PHONY: all
all: $(OUTPUT)

.PHONY: serve
serve:
	@python3 -m http.server --directory build 8080 || exit 1

# Movies
$(BUILD_DIR)/movies/index.html: $(INPUT_MOVIE_FILES) $(call templ,movies/index.html.jinja)
	@echo '$(SRC_DIR)/movies/*.json -> $@'
	@mkdir -p "$(dir $@)"
	@cat $(INPUT_MOVIE_FILES) | $(JQ_BIN) --slurp '{ entries: . }' | $(J2_BIN) -f json movies/index.html.jinja -o="$@" || exit 1

$(BUILD_DIR)/movies/%.jpg.200x0@2x.webp: $(SRC_DIR)/movies/%.jpg
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGICK_BIN) "$<" -resize 400 "$@" || exit 1

# Places
$(BUILD_DIR)/places/index.html: $(INPUT_PLACE_FILES) $(call templ,places/index.html.jinja)
	@echo '$(SRC_DIR)/places/*.json -> $@'
	@mkdir -p "$(dir $@)"
	@cat $(INPUT_PLACE_FILES) | $(JQ_BIN) --slurp '{ places: . }' | $(J2_BIN) -f json places/index.html.jinja -o="$@" || exit 1

# Cocktails
$(BUILD_DIR)/cocktails/index.html: $(INPUT_COCKTAIL_FILES)
	@echo '$(SRC_DIR)/cocktails/*.cook -> $@'
	@mkdir -p "$(dir $@)"
	@ls $(INPUT_COCKTAIL_FILES) | xargs -I {} $(COOK_BIN) recipe read --format json {} | $(JQ_BIN) --slurp '{ cocktails: . }' | $(J2_BIN) -f json cocktails/index.html.jinja -o="$@" || exit 1

$(BUILD_DIR)/cocktails/%.html: $(SRC_DIR)/cocktails/%.cook $(call templ,cocktails/_cocktail.html.jinja)
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@cat "$<" | $(COOK_BIN) recipe read --format json | $(JQ_BIN) '{ cocktail: . }' | $(J2_BIN) -f json cocktails/_cocktail.html.jinja -o="$@" || exit 1

$(BUILD_DIR)/cocktails/%.jpeg.800x0@2x.webp: $(SRC_DIR)/cocktails/%.jpeg
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGICK_BIN) "$<" -resize 1600 "$@" || exit 1

$(BUILD_DIR)/cocktails/%.jpeg.200x0@2x.webp: $(SRC_DIR)/cocktails/%.jpeg
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGICK_BIN) "$<" -resize 400 "$@" || exit 1

# Records
$(BUILD_DIR)/records/index.html: $(INPUT_RECORD_FILES) $(call templ,records/index.html.jinja)
	@echo '$(SRC_DIR)/records/*.json -> $@'
	@mkdir -p "$(dir $@)"
	@cat $(INPUT_RECORD_FILES) | $(JQ_BIN) --slurp '{ records: . }' | $(J2_BIN) -f json records/index.html.jinja -o="$@" || exit 1

$(BUILD_DIR)/records/%.jpeg.200x0@2x.webp: $(SRC_DIR)/records/%.jpeg
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGICK_BIN) "$<" -resize 400 "$@" || exit 1

# Posts
$(BUILD_DIR)/posts/index.html: $(INPUT_POST_FILES) $(call templ,posts/index.html.jinja)
	@echo '$(SRC_DIR)/posts/*.md -> $@'
	@mkdir -p "$(dir $@)"
	@ls $(INPUT_POST_FILES) | xargs -I {} sh -c './scripts/convert_md.sh {} | $(YQ_BIN) --output-format json' | $(JQ_BIN) --slurp '{ posts: . }' | $(J2_BIN) -f json posts/index.html.jinja -o="$@" || exit 1

$(BUILD_DIR)/posts/index.xml: $(INPUT_POST_FILES) $(call templ,posts/index.xml.jinja)
	@echo '$(SRC_DIR)/posts/*.md -> $@'
	@mkdir -p "$(dir $@)"
	@ls $(INPUT_POST_FILES) | xargs -I {} sh -c './scripts/convert_md.sh {} | $(YQ_BIN) --output-format json' | $(JQ_BIN) --slurp '{ posts: . }' | $(J2_BIN) -f json posts/index.xml.jinja -o="$@" || exit 1

$(BUILD_DIR)/posts/index.atom: $(INPUT_POST_FILES) $(call templ,posts/index.atom.jinja)
	@echo '$(SRC_DIR)/posts/*.md -> $@'
	@mkdir -p "$(dir $@)"
	@ls $(INPUT_POST_FILES) | xargs -I {} sh -c './scripts/convert_md.sh {} | $(YQ_BIN) --output-format json' | $(JQ_BIN) --slurp '{ posts: . }' | $(J2_BIN) -f json posts/index.atom.jinja -o="$@" || exit 1

$(BUILD_DIR)/posts/%.html: $(SRC_DIR)/posts/%.md $(call templ,posts/_post.html.jinja)
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@./scripts/convert_md.sh "$<" | $(YQ_BIN) '{ "post": . }' --output-format json | $(J2_BIN) -f json posts/_post.html.jinja -o="$@" || exit 1

$(BUILD_DIR)/posts/%.jpg.800x0@2x.webp: $(SRC_DIR)/posts/%.jpg
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGICK_BIN) "$<" -resize 1600 "$@" || exit 1

$(BUILD_DIR)/posts/%.jpeg.800x0@2x.webp: $(SRC_DIR)/posts/%.jpeg
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGICK_BIN) "$<" -resize 1600 "$@" || exit 1

$(BUILD_DIR)/posts/%.png.800x0@2x.webp: $(SRC_DIR)/posts/%.png
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGICK_BIN) "$<" -resize 1600 "$@" || exit 1

# Pages
$(BUILD_DIR)/%.html: $(SRC_DIR)/%.md $(call templ,posts/_post.html.jinja)
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@./scripts/convert_md.sh "$<" | $(YQ_BIN) '{ "post": . }' --output-format json | $(J2_BIN) -f json posts/_post.html.jinja -o="$@" || exit 1

# Assets
$(BUILD_DIR)/%: $(SRC_DIR)/%
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@cp "$<" "$@" || exit 1
