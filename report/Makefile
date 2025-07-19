PDVARS += --verbose
PDVARS += --highlight-style=pygments # the default theme

# Optional filters, must be installed to use:
#PDVARS += --filter pandoc-include
#PDVARS += --filter pandoc-codeblock-include
#PDVARS += --filter pandoc-imagine
#PDVARS += --filter pandoc-crossref

.PHONY: help
help:
	@echo "try running 'make template.pdf' ;)"

%.pdf: %.md Makefile
	pandoc -i $< ${PDVARS} -o $@ #--pdf-engine=pdflatex

%.tex: %.md Makefile
	pandoc -i $< ${PDVARS} -o $@ --standalone

%.html: %.md Makefile
	pandoc -i $< ${PDVARS} -o $@ --katex --standalone --self-contained
