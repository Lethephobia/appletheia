FROM gcr.io/google.com/cloudsdktool/google-cloud-cli:emulators

EXPOSE 8085

CMD ["gcloud", "beta", "emulators", "pubsub", "start", "--host-port=0.0.0.0:8085", "--project=banking-local"]
