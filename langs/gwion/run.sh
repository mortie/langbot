cat >input.gw
mkdir -p wd # TODO: Remove this when add-file-upload is merged
cd wd && ../bin/gwion -p../.gwplug -dSndfile ../input.gw

if [ -f gwion.wav ] && [ "$(du gwion.wav | cut -f1)" -gt 4 ]; then
	ffmpeg -loglevel error -i gwion.wav -f mp3 gwion.mp3
	ffmpeg -loglevel error -i gwion.wav -filter_complex \
		"[0:a]aformat=channel_layouts=mono, \
		compand=gain=-6, \
		showwavespic=s=600x120:colors=#9cf42f[fg]; \
		color=s=600x120:color=#44582c, \
		drawgrid=width=iw/10:height=ih/5:color=#9cf42f@0.1[bg]; \
		[bg][fg]overlay=format=auto,drawbox=x=(iw-w)/2:y=(ih-h)/2:w=iw:h=1:color=#9cf42f" \
		-frames:v 1 gwion.png
fi

rm -f gwion.wav
