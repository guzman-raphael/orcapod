# docker build ./tests/extra/example_pod/segment -t example.server.com/user/segment:1.0.0
# docker run --rm -d \
#   -l orcapod=true \
#   -l orcapod.pod_job.model_id.type=annotation \
#   -l orcapod.pod_job.model_id.name=segment \
#   -l orcapod.pod_job.model_id.version=0.1.0 \
#   --entrypoint python \
#   -e DELAY=60 \
#   -v ./tests/extra/data/images/subject.jpeg:/input/subject.jpeg:ro \
#   -v ./tests/extra/data/overlay_rgb_configs/colorblind_friendly.csv:/input/config1.csv:ro \
#   -v ./tests/extra/data/output:/output \
#   --cpus=0.5 \
#   --memory=2g \
#   example.server.com/user/segment:1.0.0 \
#   /run.py
# docker ps -f 'label=org.orcapod=true'
FROM maclucky/bambulab-timelapse-downloader:1.0.9
COPY ./run.py /run.py
USER root
CMD ["python3", "/run.py"]
