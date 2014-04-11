FOLDERS :=first sorting trees
CLEAN_FOLDERS := $(patsubst %, clean_%, $(FOLDERS))

all: $(FOLDERS)

clean: $(CLEAN_FOLDERS)

define FOLDER_TARGET
$(1):
	cd $(1) && make

clean_$(1):
	cd $(1) && make clean
endef

$(foreach folder,$(FOLDERS),$(eval $(call FOLDER_TARGET,$(folder))))

.PHONY: all clean $(FOLDERS) $(CLEAN_FOLDERS)
