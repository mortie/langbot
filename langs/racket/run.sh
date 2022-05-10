cat >input.rkt.in
if head -n 1 input.rkt.in | grep '^\s*#' >/dev/null; then
	mv input.rkt.in input.rkt
else
	echo '#lang racket' >input.rkt
	cat input.rkt.in >>input.rkt
	rm input.rkt.in
fi

cd wd && exec racket ../input.rkt
