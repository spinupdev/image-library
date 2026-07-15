# Friendly prompt: depot assigns each guest a short internal hostname (e.g.
# "197") that this image doesn't control. Override PS1 instead of relying on
# \h so the prompt stays readable ("zeish ~ $") regardless of what depot sets.
if [ -n "$BASH_VERSION" ]; then
  PS1='\[\033[1;36m\]zeish\[\033[0m\] \[\033[1;34m\]\w\[\033[0m\] \$ '
fi
