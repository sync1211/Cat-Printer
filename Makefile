version= $(shell cat version)

prepare:
	$(info Preparing build...)

	$(info Convert zh-CN to zh-TW...)
	$(cd www/lang && shell sed 's/中文（简体）/中文（臺灣正體）/' < zh-CN.json | opencc -c s2twp.json > zh-TW.json)

	$(info Bundling scripts via tsc...)
	$(shell cd www && npx tsc  --allowJs  --outFile main.comp.js  $$(cat all_js.txt))

bundle-windows: prepare
	$(info Creating bundle for Windows....)
	cd ./build-common; \
	python3 ./bundle.py -w $(version) 

bundle-linux: prepare
	$(info Creating bundle for linux...)
	cd ./build-common; \
	python3 ./bundle.py $(version)

bundle-bare: prepare
	$(info Creating bare bundle...)
	cd ./build-common; \
	python3 ./bundle.py -b $(version)

bundle-all: bundle-windows bundle-linux bundle-bare 

#apk: clean-apk bundle-bare
#	$(info Building release APK...)
#	cd build-android; \
#	unzip -q "./cat-printer-bare-$(version).zip"; \
#	mv "cat-printer" "build-dist"; \
#	p4a apk --private "dist" --dist_name="cat-printer" --package="io.github.naitlee.catprinter" --name="Cat Printer" \
#		--icon=icon.png --version="$(version)" --bootstrap=webview --window --requirements="`cat build-deps.txt`" \
#		--blacklist-requirements=sqlite3,openssl --port=8095 --arch=arm64-v8a --release \
#		--presplash=blank.png --presplash-color=black --add-source="advancedwebview" --orientation=user \
#		--permission=BLUETOOTH --permission=BLUETOOTH_SCAN --permission=BLUETOOTH_CONNECT \
#		--permission=BLUETOOTH_ADMIN --permission=ACCESS_FINE_LOCATION --permission=ACCESS_COARSE_LOCATION $@
#
#	$(MAKE) apk-sign
#
#apk-sign:
#	cd build-android; \
#
#	$(shell $ANDROIDSDK/build-tools/*/zipalign 4 "cat-printer-release-unsigned-$(version)-.apk" $signed_apk); \
#	$(shell $ANDROIDSDK/build-tools/*/apksigner sign --ks $1 "cat-printer-android-$(version).apk"); \
#	$(info Build complete! Moving APK...); \
#	mv $signed_apk $signed_apk.idsig ../; \
#
#clean-apk:
#	$(info Removing build APK files...)
#	rm -rf build-android/dist
#	rm -f build-android/*.apk
#
#
#build-android: prepare
#	$(info Building android version...)
#	p4a apk --private .. --dist_name="cat-printer" --package="io.github.naitlee.catprinter" --name="Cat Printer" \
#		--icon=icon.png --version=$(version) --bootstrap=webview --window --requirements="`cat build-deps.txt`" \
#		--blacklist-requirements=sqlite3,openssl --port=8095 --arch=arm64-v8a --blacklist="blacklist.txt" \
#		--presplash=blank.png --presplash-color=black --add-source="advancedwebview" --orientation=user \
#		--permission=BLUETOOTH --permission=BLUETOOTH_SCAN --permission=BLUETOOTH_CONNECT \
#		--permission=BLUETOOTH_ADMIN --permission=ACCESS_FINE_LOCATION --permission=ACCESS_COARSE_LOCATION $@

clean-python:
	$(info Removing Python cache files...)
	find . -type d -name __pycache__ -exec rm -r {} \+
	find . -type f -name *.pyc -exec rm {} \+
clean-bundles:
	$(info Removing bundles...)
	rm -f cat-printer-*.zip
	rm -f cat-printer-sha256-*.txt

clean: clean-bundles clean-python
	rm -f cat-printer-*.apk*

.DEFAULT_GOAL=bundle-linux
