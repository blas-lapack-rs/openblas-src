source := OpenBLAS

unexport TARGET

ifdef CARGO_FEATURE_EXCLUDE_CBLAS
cblas := NO
else
cblas := YES
endif

all:
	$(MAKE) -C $(source) libs netlib shared -j$(NUM_JOBS:=1) $(cblas)_CBLAS=1
	$(MAKE) -C $(source) install DESTDIR=$(OUT_DIR)
