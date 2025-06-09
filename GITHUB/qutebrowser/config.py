config.load_autoconfig()

# Open video link in mpv (hover a link and press 'M')
config.bind('M', 'hint links spawn --detach mpv {hint-url}')

# Open current page URL in mpv (useful on YouTube video page)
config.bind('Y', 'spawn --detach mpv {url}')

# a bind to clear cache and reload
config.bind('<Ctrl-r>', 'history-clear ;; reload')

# include web-anti-youtube-ads needs work
#config.set('content.user_scripts', ['~/.config./qutebrowser/greasemonkey/yt-ads.js'])

# darkmod
c.colors.webpage.darkmode.enabled = True
c.colors.webpage.darkmode.algorithm = 'lightness-cielab' # smooth, natural
c.colors.webpage.darkmode.policy.images = 'never' # avoid invert image
