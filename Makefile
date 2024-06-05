# in and out
SRC_DIR := assets
BUILD_DIR := build

# deps
PANDOC_BIN := pandoc
IMAGEMAGIC_BIN := magick
JQ_BIN := jq
YQ_BIN := yq
J2_BIN := j2 --customize ./j2_customize.py
COOK_BIN := cook

# find input files by type
INPUT_MD_FILES := $(shell find $(SRC_DIR) -type f -name '*.md')
INPUT_POST_FILES := $(shell find $(SRC_DIR)/posts -type f -name '*.md')
INPUT_POST_JPG_IMAGE_FILES := $(shell find $(SRC_DIR)/posts -type f -name '*.jpg')
INPUT_POST_JPEG_IMAGE_FILES := $(shell find $(SRC_DIR)/posts -type f -name '*.jpeg')
INPUT_POST_PNG_IMAGE_FILES := $(shell find $(SRC_DIR)/posts -type f -name '*.png')
INPUT_MOVIE_FILES := $(shell find $(SRC_DIR)/movies -type f -name '*.json')
INPUT_MOVIE_IMAGE_FILES := $(shell find $(SRC_DIR)/movies -type f -name '*.jpg')
INPUT_RECORD_FILES := $(shell find $(SRC_DIR)/records -type f -name '*.json')
INPUT_RECORD_IMAGE_FILES := $(shell find $(SRC_DIR)/records -type f -name '*.jpeg')
INPUT_COCKTAIL_FILES := $(shell find $(SRC_DIR)/cocktails -type f -name '*.cook')
INPUT_COCKTAIL_IMAGE_FILES := $(shell find $(SRC_DIR)/cocktails -type f -name '*.jpeg')
INPUT_PLACE_FILES := $(shell find $(SRC_DIR)/places -type f -name '*.json')

# calculate all other input files
INPUT_FILES := $(INPUT_MD_FILES)
INPUT_FILES := $(INPUT_FILES) $(INPUT_POST_FILES)
INPUT_FILES := $(INPUT_FILES) $(INPUT_POST_JPG_IMAGE_FILES)
INPUT_FILES := $(INPUT_FILES) $(INPUT_POST_JPEG_IMAGE_FILES)
INPUT_FILES := $(INPUT_FILES) $(INPUT_POST_PNG_IMAGE_FILES)
INPUT_FILES := $(INPUT_FILES) $(INPUT_MOVIE_FILES)
INPUT_FILES := $(INPUT_FILES) $(INPUT_MOVIE_IMAGE_FILES)
INPUT_FILES := $(INPUT_FILES) $(INPUT_RECORD_FILES)
INPUT_FILES := $(INPUT_FILES) $(INPUT_RECORD_IMAGE_FILES)
INPUT_FILES := $(INPUT_FILES) $(INPUT_COCKTAIL_FILES)
INPUT_FILES := $(INPUT_FILES) $(INPUT_COCKTAIL_IMAGE_FILES)
INPUT_FILES := $(INPUT_FILES) $(INPUT_PLACE_FILES)

INPUT_ALL_FILES := $(shell find $(SRC_DIR) -type f)
INPUT_OTHER_FILES := $(filter-out $(INPUT_FILES),$(INPUT_ALL_FILES))

# calculate output files
OUTPUT_MD_FILES := $(patsubst $(SRC_DIR)/%.md,$(BUILD_DIR)/%.html,$(INPUT_MD_FILES))
OUTPUT_POST_JPG_IMAGE_FILES := $(patsubst $(SRC_DIR)/%.jpg,$(BUILD_DIR)/%.jpg.800x0@2x.webp,$(INPUT_POST_JPG_IMAGE_FILES))
OUTPUT_POST_JPEG_IMAGE_FILES := $(patsubst $(SRC_DIR)/%.jpeg,$(BUILD_DIR)/%.jpeg.800x0@2x.webp,$(INPUT_POST_JPEG_IMAGE_FILES))
OUTPUT_POST_PNG_IMAGE_FILES := $(patsubst $(SRC_DIR)/%.png,$(BUILD_DIR)/%.png.800x0@2x.webp,$(INPUT_POST_PNG_IMAGE_FILES))
OUTPUT_POST_IMAGE_FILES := $(OUTPUT_POST_JPG_IMAGE_FILES) $(OUTPUT_POST_JPEG_IMAGE_FILES) $(OUTPUT_POST_PNG_IMAGE_FILES)
OUTPUT_IMAGE_FILES := $(patsubst $(SRC_DIR)/%.md,$(BUILD_DIR)/%.html,$(INPUT_IMAGE_FILES))
OUTPUT_MOVIE_IMAGE_FILES := $(patsubst $(SRC_DIR)/%.jpg,$(BUILD_DIR)/%.jpg.70x0@2x.webp,$(INPUT_MOVIE_IMAGE_FILES))
OUTPUT_RECORD_FILES := $(patsubst $(SRC_DIR)/%.json,$(BUILD_DIR)/%.html,$(INPUT_RECORD_FILES))
OUTPUT_RECORD_IMAGE_FILES := $(patsubst $(SRC_DIR)/%.jpeg,$(BUILD_DIR)/%.jpeg.200x0@2x.webp,$(INPUT_RECORD_IMAGE_FILES))
OUTPUT_COCKTAIL_FILES := $(patsubst $(SRC_DIR)/%.cook,$(BUILD_DIR)/%.html,$(INPUT_COCKTAIL_FILES))
OUTPUT_COCKTAIL_SMALL_IMAGE_FILES := $(patsubst $(SRC_DIR)/%.jpeg,$(BUILD_DIR)/%.jpeg.200x0@2x.webp,$(INPUT_COCKTAIL_IMAGE_FILES))
OUTPUT_COCKTAIL_LARGE_IMAGE_FILES := $(patsubst $(SRC_DIR)/%.jpeg,$(BUILD_DIR)/%.jpeg.800x0@2x.webp,$(INPUT_COCKTAIL_IMAGE_FILES))
OUTPUT_COCKTAIL_IMAGE_FILES := $(OUTPUT_COCKTAIL_LARGE_IMAGE_FILES) $(OUTPUT_COCKTAIL_SMALL_IMAGE_FILES)
OUTPUT_PLACE_FILES := $(patsubst $(SRC_DIR)/%.json,$(BUILD_DIR)/%.html,$(INPUT_PLACE_FILES))
OUTPUT_OTHER_FILES := $(patsubst $(SRC_DIR)/%,$(BUILD_DIR)/%,$(INPUT_OTHER_FILES))

OUTPUT := $(OUTPUT_MD_FILES)
OUTPUT := $(OUTPUT) $(OUTPUT_COCKTAIL_FILES)
OUTPUT := $(OUTPUT) $(BUILD_DIR)/cocktails/index.html
OUTPUT := $(OUTPUT) $(OUTPUT_COCKTAIL_IMAGE_FILES)
OUTPUT := $(OUTPUT) $(BUILD_DIR)/records/index.html
OUTPUT := $(OUTPUT) $(BUILD_DIR)/movies/index.html
OUTPUT := $(OUTPUT) $(OUTPUT_MOVIE_IMAGE_FILES)
OUTPUT := $(OUTPUT) $(OUTPUT_RECORD_IMAGE_FILES)
OUTPUT := $(OUTPUT) $(BUILD_DIR)/posts/index.html
OUTPUT := $(OUTPUT) $(OUTPUT_POST_IMAGE_FILES)
OUTPUT := $(OUTPUT) $(BUILD_DIR)/places/index.html
OUTPUT := $(OUTPUT) $(OUTPUT_OTHER_FILES)

.PHONY: all
all: $(OUTPUT)

.PHONY: serve
serve:
	@python3 -m http.server --directory build 8080

# movies
$(BUILD_DIR)/movies/index.html:
	@echo '$(SRC_DIR)/movies/*.json -> $@'
	@mkdir -p "$(dir $@)"
	@cat $(INPUT_MOVIE_FILES) | $(JQ_BIN) --slurp '{ entries: . }' | $(J2_BIN) -f json movies/index.html.jinja -o="$@"

$(BUILD_DIR)/movies/%.jpg.70x0@2x.webp: $(SRC_DIR)/movies/%.jpg
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGIC_BIN) "$<" -resize 140 "$@"

# places
$(BUILD_DIR)/places/index.html:
	@echo '$(SRC_DIR)/places/*.json -> $@'
	@mkdir -p "$(dir $@)"
	@cat $(INPUT_PLACE_FILES) | $(JQ_BIN) --slurp '{ places: . }' | $(J2_BIN) -f json places/index.html.jinja -o="$@"

# cocktails
$(BUILD_DIR)/cocktails/index.html:
	@echo '$(SRC_DIR)/cocktails/*.cook -> $@'
	@mkdir -p "$(dir $@)"
	@ls $(INPUT_COCKTAIL_FILES) | xargs -L 1 $(COOK_BIN) recipe read --output-format json | $(JQ_BIN) --slurp '{ cocktails: . }' | $(J2_BIN) -f json cocktails/index.html.jinja -o="$@"

$(BUILD_DIR)/cocktails/%.html: $(SRC_DIR)/cocktails/%.cook
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@cat "$<" | $(COOK_BIN) recipe read --output-format json | $(JQ_BIN) '{ cocktail: . }' | $(J2_BIN) -f json cocktails/_cocktail.html.jinja -o="$@"

$(BUILD_DIR)/cocktails/%.jpeg.800x0@2x.webp: $(SRC_DIR)/cocktails/%.jpeg
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGIC_BIN) "$<" -resize 1600 "$@"

$(BUILD_DIR)/cocktails/%.jpeg.200x0@2x.webp: $(SRC_DIR)/cocktails/%.jpeg
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGIC_BIN) "$<" -resize 400 "$@"

# records
$(BUILD_DIR)/records/index.html:
	@echo '$(SRC_DIR)/records/*.json -> $@'
	@mkdir -p "$(dir $@)"
	@cat $(INPUT_RECORD_FILES) | $(JQ_BIN) --slurp '{ records: . }' | $(J2_BIN) -f json records/index.html.jinja -o="$@"

$(BUILD_DIR)/records/%.jpeg.200x0@2x.webp: $(SRC_DIR)/records/%.jpeg
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGIC_BIN) "$<" -resize 400 "$@"

# posts
$(BUILD_DIR)/posts/index.html:
	@echo '$(SRC_DIR)/posts/*.md -> $@'
	@mkdir -p "$(dir $@)"
	@ls $(INPUT_POST_FILES) | xargs -L 1 -I {} sh -c './scripts/convert_md.sh {} | $(YQ_BIN) --output-format json' | $(JQ_BIN) --slurp '{ posts: . }' | $(J2_BIN) -f json posts/index.html.jinja -o="$@"

$(BUILD_DIR)/posts/%.html: $(SRC_DIR)/%.md
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@./scripts/convert_md.sh "$<" | $(YQ_BIN) '{ "post": . }' | $(J2_BIN) -f yaml posts/_post.html.jinja -o="$@"

$(BUILD_DIR)/posts/%.jpg.800x0@2x.webp: $(SRC_DIR)/posts/%.jpg
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGIC_BIN) "$<" -resize 1600 "$@"

$(BUILD_DIR)/posts/%.jpeg.800x0@2x.webp: $(SRC_DIR)/posts/%.jpeg
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGIC_BIN) "$<" -resize 1600 "$@"

$(BUILD_DIR)/posts/%.png.800x0@2x.webp: $(SRC_DIR)/posts/%.png
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@$(IMAGEMAGIC_BIN) "$<" -resize 1600 "$@"

# pages
$(BUILD_DIR)/%.html: $(SRC_DIR)/%.md
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@./scripts/convert_md.sh "$<" | $(YQ_BIN) '{ "post": . }' | $(J2_BIN) -f yaml posts/_post.html.jinja -o="$@"

# assets
$(BUILD_DIR)/%: $(SRC_DIR)/%
	@echo '$< -> $@'
	@mkdir -p "$(dir $@)"
	@cp "$<" "$@"

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)
