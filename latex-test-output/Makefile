LATEXMK    = latexmk
LATEXMKOPT = -xelatex -bibtex -shell-escape -synctex=1 -interaction=nonstopmode -file-line-error
OUTDIR = out
DOCUMENT = main

build:
	$(LATEXMK) $(LATEXMKOPT) \
		-output-directory=$(OUTDIR) \
		$(DOCUMENT).tex
	mv $(OUTDIR)/$(DOCUMENT).pdf $(DOCUMENT).pdf