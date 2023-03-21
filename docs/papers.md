
# Papers

## Decoupling Language and Editor - The Impact of the Language Server Protocol on Textual Domain-Specific Languages

* About decoupling a DSL from a specific editor by using LSP
* References Spoofax :)
* 

```bibtex
@inproceedings{bunder2019decoupling,
title={Decoupling Language and Editor-The Impact of the Language Server Protocol on Textual Domain-Specific Languages.},
author={B{\"u}nder, Hendrik},
booktitle={MODELSWARD},
pages={129--140},
year={2019}
}
```

## Towards Multi-editor Support for Domain-Specific Languages Utilizing the Language Server Protocol

* Another SWOT (bünder again I guess)

```bibtex
@inproceedings{bunder2020towards,
title={Towards multi-editor support for domain-specific languages utilizing the language server protocol},
author={B{\"u}nder, Hendrik and Kuchen, Herbert},
booktitle={Model-Driven Engineering and Software Development: 7th International Conference, MODELSWARD 2019, Prague, Czech Republic, February 20--22, 2019, Revised Selected Papers 7},
pages={225--245},
year={2020},
organization={Springer}
}
```

## Domain-Specific Languages for Composable Editor Plugins

* Eelco's paper about spoofax

```bibtex
@article{kats2010domain,
title={Domain-specific languages for composable editor plugins},
author={Kats, Lennart CL and Kalleberg, Karl T and Visser, Eelco},
journal={Electronic Notes in Theoretical Computer Science},
volume={253},
number={7},
pages={149--163},
year={2010},
publisher={Elsevier}
}
```


## Language-parametric static semantic code completion

TODO: make nicer
* Only about code completion. Useless for me?

```bibtex
@article{pelsmaeker2022language,
title={Language-parametric static semantic code completion},
author={Pelsmaeker, Dani{\"e}l AA and van Antwerpen, Hendrik and Poulsen, Casper Bach and Visser, Eelco},
journal={Proceedings of the ACM on Programming Languages},
volume={6},
number={OOPSLA1},
pages={1--30},
year={2022},
publisher={ACM New York, NY, USA}
}
```

## Towards Language-Parametric Refactorings

* Related to Spoofax
* refactorings based on the spoofax definition of the language, which then work for any language made in spoofax
* Using the scope graph
* Tiny! 2 pages haha
* 

```bibtex
@inproceedings{misteli2020towards,
title={Towards language-parametric refactorings},
author={Misteli, Philippe D},
booktitle={Companion Proceedings of the 4th International Conference on Art, Science, and Engineering of Programming},
pages={213--214},
year={2020}
}
```

https://dl.acm.org/doi/pdf/10.1145/3397537.3398476

## Experience with ANSI C markup language for a cross-referencer

* They make a cross referencer
* their question: How to store the info needed for cross referencing c programs in xml?
* Lots has changed since 2002
* Their approach:
  * Store the entire syntax tree of c in XML so it has to be parsed only once
  * Try to unify redundancies (`unsigned long == long unsigned`) adding non-canonical notation
* Mentions lots of approaches others took to accomplish this: worth a look, though old
  * 

https://ieeexplore-ieee-org.tudelft.idm.oclc.org/stamp/stamp.jsp?tp=&arnumber=1174891

## Portable Editor Services

* AESI: maximal interface supported by all editors explored with minimum requirements for implementors
* Starts with adapting spoofax to jetbrains (which seems like they experienced some issues)
* Then goes into what editor plugins are and compares how xtext generates plugins for a number of editors?
* Comparing editor services:
  * Derives 13 editor services (from `erdweg2013state`) expected in editors, and lists how they are supported across common editors
  * Ranks the editors in a ternary manner, as also seen in other related papers
  * Good description of how exactly syntax colouring works in different editors, including textmate's interesting system with classes
  * Interestingly: mentions ctags
* Chapter 5: Adaptable Editor Services Interface
  * Refers to `erdweg2013state` which seems useful
  * 
* Conclusion clearly mentions contributions, which seems useful

https://repository.tudelft.nl/islandora/object/uuid:c8b554de-bcb6-4896-a9bf-c03cca37e344/datastream/OBJ/download

@article{pelsmaeker2018portable,
title={Portable Editor Services},
author={Pelsmaeker, Dani{\"e}l},
year={2018}
}

## The State of the Art in Language Workbenches

* Has an awesome tree of important features of a language workbench
  * Which of these features translate to useful code exploration services?
* 

https://homepages.cwi.nl/~storm/publications/lwc13paper.pdf

```bibtex
@inproceedings{erdweg2013state,
title={The state of the art in language workbenches: Conclusions from the language workbench challenge},
author={Erdweg, Sebastian and Van Der Storm, Tijs and V{\"o}lter, Markus and Boersma, Meinte and Bosman, Remi and Cook, William R and Gerritsen, Albert and Hulshout, Angelo and Kelly, Steven and Loh, Alex and others},
booktitle={Software Language Engineering: 6th International Conference, SLE 2013, Indianapolis, IN, USA, October 26-28, 2013. Proceedings 6},
pages={197--217},
year={2013},
organization={Springer}
}
```
