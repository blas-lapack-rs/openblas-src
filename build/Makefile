build := $(CARGO_MANIFEST_DIR)/build
source := $(build)/OpenBLAS
patches := $(build)/patches

unexport TARGET

ifdef CARGO_FEATURE_EXCLUDE_CBLAS
cblas := NO
else
cblas := YES
endif

all: $(source)/libopenblas.a
	$(MAKE) -C $(source) install DESTDIR=$(OUT_DIR)

$(source)/libopenblas.a:
	(cd $(source) && (cat $(patches)/*.patch | patch -p1))
	$(MAKE) -C $(source) libs netlib shared -j$(NUM_JOBS:=1) $(cblas)_CBLAS=1
	(cd $(source) && (cat $(patches)/*.patch | patch -p1 -R))
