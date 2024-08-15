# You can find the new timestamped tags here: https://hub.docker.com/r/gitpod/workspace-full/tags
FROM gitpod/workspace-full-vnc

# Install custom tools, runtime, etc.
# install-packages is a wrapper for `apt` that helps skip a few commands in the docker env.

RUN sudo apt update && \
    sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

    
# RUN sudo apt-get update && \
#     sudo apt-get install -y libgtk-3-dev && \
#     sudo rm -rf /var/lib/apt/lists/*


# RUN apt-get update \
#     && apt-get install -y libgtk-3-dev
# 
# RUN apt-get update \
#  && apt-get install -y libx11-dev libxkbfile-dev libsecret-1-dev libgconf2–4 libnss3

# Apply user-specific settings
