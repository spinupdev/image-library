# nvm (Node), pyenv (Python), g (Go) — available in login and `docker exec` shells.
export NVM_DIR="/home/user/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

export PYENV_ROOT="/home/user/.pyenv"
export GOROOT="/home/user/.go"
export GOPATH="/home/user/go"
export PATH="$PYENV_ROOT/bin:$PYENV_ROOT/shims:$GOROOT/bin:$GOPATH/bin:/home/user/.local/bin:/home/user/.grok/bin:$PATH"

command -v pyenv >/dev/null 2>&1 && eval "$(pyenv init -)"
