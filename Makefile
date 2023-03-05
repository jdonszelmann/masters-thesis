LATEXMK    = latexmk
LATEXMKOPT = -xelatex -bibtex -shell-escape -synctex=1 -interaction=nonstopmode -file-line-error
CONTINUOUS = -pvc -view=default -halt-on-error
ROOT = $(shell pwd)
WORKDIR    := $(ROOT)/latex

DOCUMENT   := document
OUTPUT     := $(DOCUMENT)
RESEARCHR  := b9363f9e-a672-4c96-9cea-9efcaf813499-msc-thesis
SRCDIR     := $(WORKDIR)/src
FIGDIR     := $(WORKDIR)/src/fig
IMGDIR     := $(WORKDIR)/src/img
RESDIR     := $(WORKDIR)/src/res
OUTDIR     := $(WORKDIR)/out
SRCBIB     := $(SRCDIR)/researchr.bib
SRCS       := Makefile $(wildcard $(SRCDIR)/*.tex) $(shell find $(FIGDIR)/* $(IMGDIR)/* $(RESDIR)/* -type f) $(SRCBIB)
IMGS       := $(wildcard $(IMGDIR)/*.ps) $(wildcard $(IMGDIR)/*.eps)
OBJS       := $(wildcard $(OUTDIR)/*.aux) $(wildcard $(OUTDIR)/*.bbl) $(wildcard $(OUTDIR)/*.pdf)

.PHONY: all clean .refresh view show bib clean-bib

all: $(OUTDIR)/$(DOCUMENT).pdf

.refresh:
	touch .refresh

$(OUTDIR):
	mkdir -p $(OUTDIR)
	sudo mount -t tmpfs -o size=1g tmpfs $(OUTDIR)

$(OUTDIR)/$(DOCUMENT).pdf: .refresh $(SRCS) | $(OUTDIR) $(SRCBIB)
	cd $(SRCDIR)/ && \
		$(LATEXMK) $(LATEXMKOPT) \
			-output-directory=$(OUTDIR) \
			$(DOCUMENT)
	mv $(OUTDIR)/$(DOCUMENT).pdf $(OUTPUT).pdf
	-#mv $(OUTDIR)/$(DOCUMENT).vtc $(OUTPUT).vtc
	-#mv $(OUTDIR)/$(DOCUMENT).synctex.gz $(OUTPUT).synctex.gz

watch: $(SRCBIB) | $(OUTDIR)
	cd $(SRCDIR)/ && \
		$(LATEXMK) $(LATEXMKOPT) $(CONTINUOUS) \
			-output-directory=$(OUTDIR) \
			$(DOCUMENT)

bib: clean-bib $(SRCBIB) fix-bib

$(SRCBIB):
	curl https://researchr.org/downloadbibtex/bibliography/$(RESEARCHR)/ -o $(SRCBIB)

fix-bib: $(SRCBIB)
	sed -i '1 s/^/% /' $(SRCBIB)
	sed -i 's/doi = {http.*\/\(10\..*\)}/doi = {\1}/' $(SRCBIB)
	sed -i '/doi = {http.*}/d' $(SRCBIB)

clean-bib:
	rm -f $(SRCBIB)

clean: clean-bib
	rm -rf $(OUTDIR)/*
	-sudo umount -R $(OUTDIR)
	rm -rf $(OUTDIR)
	rm -f $(WORKDIR)/*.aux
	rm -f $(WORKDIR)/*.bbl
	rm -f $(WORKDIR)/*.blg
	rm -f $(WORKDIR)/*.fdb_latexmk
	rm -f $(WORKDIR)/*.fls
	rm -f $(WORKDIR)/*.log
	rm -f $(WORKDIR)/*.out
	rm -f $(WORKDIR)/*.pdf
	rm -f $(WORKDIR)/*.swp
	rm -f $(WORKDIR)/*.synctex.gz
	rm -f $(WORKDIR)/*.vtc
	rm -f $(WORKDIR)/src/*.aux
	rm -f $(WORKDIR)/src/*.bbl
	rm -f $(WORKDIR)/src/*.blg
	rm -f $(WORKDIR)/src/*.fdb_latexmk
	rm -f $(WORKDIR)/src/*.fls
	rm -f $(WORKDIR)/src/*.log
	rm -f $(WORKDIR)/src/*.out
	rm -f $(WORKDIR)/src/*.pdf
	rm -f $(WORKDIR)/src/*.swp
	rm -f $(WORKDIR)/src/*.synctex.gz
	rm -f $(WORKDIR)/src/*.vtc

show: view
view: all
	open $(OUTPUT).pdf
