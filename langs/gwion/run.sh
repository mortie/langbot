cat >input.gw
cd wd && ../bin/gwion -p../.gwplug -dSndfile ../input.gw 2>../stderr.log
if [ -s ../stderr.log ]; then
	cat ../stderr.log >&2
	rm -f gwion.wav
	exit 1
fi

find . -type f -name '*.wav' | while read -r wavf; do
	if [ "$(du "$wavf" | cut -f1)" -gt 4 ]; then
		mp3f="${wavf%.wav}.mp3"
		pngf="${wavf%.wav}.png"
		ffmpeg -loglevel error -i "$wavf" -f mp3 "$mp3f"
		ffmpeg -loglevel error -i "$wavf" -filter_complex \
			"[0:a]aformat=channel_layouts=mono, \
			compand=gain=-6, \
			showwavespic=s=600x120:colors=#9cf42f[fg]; \
			color=s=600x120:color=#44582c, \
			drawgrid=width=iw/10:height=ih/5:color=#9cf42f@0.1[bg]; \
			[bg][fg]overlay=format=auto,drawbox=x=(iw-w)/2:y=(ih-h)/2:w=iw:h=1:color=#9cf42f" \
			-frames:v 1 "$pngf"
	fi
	rm -f "$wavf"
done

find . -type f -name '*.bmp' | while read -r bmpf; do
	pngf="${bmpf%.bmp}.png"
	ffmpeg -loglevel error -i "$bmpf" "$pngf"
	rm -f "$ppmf"
done

find . -type f -name '*.ppm' | while read -r ppmf; do
	pngf="${ppmf%.ppm}.png"
	ffmpeg -loglevel error -i "$ppmf" "$pngf"
	rm -f "$ppmf"
done
