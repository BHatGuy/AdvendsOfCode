TOPTARGETS := all clean test benchmark

SUBDIRS := $(subst /.,, $(wildcard */.))


$(TOPTARGETS): $(SUBDIRS)
$(SUBDIRS):
	@echo $@ $(MAKECMDGOALS)
	@$(MAKE) --no-print-directory -C $@ $(MAKECMDGOALS) || :
	@echo

.PHONY: $(TOPTARGETS) $(SUBDIRS)
