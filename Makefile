# latex
LATEXMK?=latexmk
BBL_EXT?=bbl

# figures
FIGURES_DIR?=figures
MMDC?=mmdc
PDFCROP?=pdfcrop
PDFCROP_MARGINS?=10
MMD_EXT?=mmd
MMD_OUT_EXT?=pdf

MMD_SOURCES:=$(wildcard $(FIGURES_DIR)/*.$(MMD_EXT))
MMD_TARGETS:=$(MMD_SOURCES:.$(MMD_EXT)=.$(MMD_OUT_EXT))

.PHONY: all build clean \
	build-latex clean-latex \
	build-figures \
	build-mmd clean-mmd

all: build

build: build-latex build-mmd

clean: clean-latex clean-mmd

# latex
build-latex: build-figures
	$(LATEXMK)

build-figures: build-mmd

clean-latex:
	$(LATEXMK) -C
	rm -f *.$(BBL_EXT)

# mermaid
build-mmd: $(MMD_TARGETS)

%.$(MMD_OUT_EXT): %.$(MMD_EXT)
	$(MMDC) -i $< -o $@
	$(PDFCROP) --margins $(PDFCROP_MARGINS) $@ $@

clean-mmd:
	rm -f $(MMD_TARGETS)
