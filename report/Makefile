PANDOC_ARGS += --verbose
PANDOC_ARGS += --highlight-style=pygments # the default theme
PANDOC_ARGS += -f markdown-implicit_figures

# Optional filters, needs be installed seperately:

#PANDOC_ARGS += --filter pandoc-include
#PANDOC_ARGS += --filter pandoc-include-code
#PANDOC_ARGS += --filter pandoc-imagine
#PANDOC_ARGS += --filter pandoc-crossref
#PANDOC_ARGS += --filter include-files.lua      # this only works in the nix environment
#PANDOC_ARGS += --filter include-code-files.lua # this only works in the nix environment

.PHONY: help
help:
	@echo "try running 'make template.pdf'"

.PHONY: has-%
has-%:
	@command -v $* >/dev/null || ( \
		echo "ERROR: Command '$*' not found! Make sure it is installed and available in PATH"; \
		false; \
	) >&2

%.pdf: %.md Makefile | has-pandoc
	pandoc -i $< ${PANDOC_ARGS} -o $@ #--pdf-engine=pdflatex

%.tex: %.md Makefile | has-pandoc
	pandoc -i $< ${PANDOC_ARGS} -o $@ --standalone

%.html: %.md Makefile | has-pandoc
	pandoc -i $< ${PANDOC_ARGS} -o $@ --katex --standalone --self-contained
