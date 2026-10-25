package main

import (
	"log/slog"
	"os"
)

func main() {
	// Drop the time so this page's output stays stable.
	logger := slog.New(slog.NewTextHandler(os.Stdout, &slog.HandlerOptions{
		ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr {
			if a.Key == slog.TimeKey {
				return slog.Attr{}
			}
			return a
		},
	}))

	logger.Info("server starting", "port", 3000)

	req := logger.With("method", "GET", "path", "/notes")
	req.Info("listed notes", "count", 2)
	req.Warn("slow response", "user", "ana")
}
