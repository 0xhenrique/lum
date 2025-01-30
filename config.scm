;; (define-module (gnu packages myproject)
;;   #:use-module (guix packages)
;;   #:use-module (guix download)
;;   #:use-module (gnu packages licenses)
;;   #:use-module (guix git-download)
;;   #:use-module (guix build-system gnu)
;;   #:use-module (gnu packages java)
;;   #:use-module (guix build-system copy)
;;   #:use-module (guix utils))

;; (define-public lum
;;   (package
;;     (name "lum")   ; Change this to your project's name
;;     (version "1.0.0")            ; Change to your project's version
;;     (source
;;      (origin
;;        (method git-fetch)         ; or url-fetch if you have a tarball
;;        (uri (git-reference
;;              (url "https://github.com/0xhenrique/lum") ; Change to your repo URL
;;              (commit "master")))                     ; Use a specific commit or branch
;;        (file-name (git-file-name name version))
;;        (sha256
;;         (base32 "087bvbx056axg0z5m27mqa9rqb1k242gqk3msgwwy3kldy1i736j")))) ;; Replace with actual sha256

;;     (build-system gnu-build-system)  ;; Use gnu-build-system
;;     (arguments
;;      `(#:phases
;;        (modify-phases %standard-phases
;;          (add-before 'build 'build-with-lein
;;            (lambda* (#:key inputs #:allow-other-keys)
;;              (setenv "PATH" (string-append (assoc-ref inputs "leiningen") "/bin:" (getenv "PATH")))
;;              (invoke "lein" "uberjar"))))))  ;; Run lein uberjar to build the project

;;     (inputs
;;      `(("leiningen" ,leiningen)))  ;; Ensure Leiningen is in inputs

;;     (native-inputs
;;      `(("git" ,git)))              ;; Optional, if you're using git

;;     (home-page "https://github.com/0xhenrique/lum")  ;; Change to your project's home page
;;     (synopsis "A sample Clojure project packaged with Guix")  ;; Change to your project's synopsis
;;     (description
;;      "This package provides a Clojure project that uses Leiningen to build.")  ;; Change to your project description
;;     (license license:asl2.0)))  ;; Change to the correct license if necessary

(define-module (gnu packages figlet)
  #:use-module ((guix licenses) #:prefix license:)
  #:use-module (guix packages)
  #:use-module (guix download)
  #:use-module (guix git-download)
  #:use-module (guix leiningen)
  #:use-module (guix build-system gnu))

(define-public lum
  (package
    (name "lum")
    (version "1.0.0")
    (source
     (origin
       (method git-fetch)         ; or url-fetch if you have a tarball
       (uri (git-reference
             (url "https://github.com/0xhenrique/lum") ; Change to your repo URL
             (commit "master")))                     ; Use a specific commit or branch
       (file-name (git-file-name name version))
       (sha256
        (base32 "087bvbx056axg0z5m27mqa9rqb1k242gqk3msgwwy3kldy1i736j")))) ;; Replace with actual sha256
    (build-system gnu-build-system)
    (arguments
     `(#:phases
       (modify-phases %standard-phases (delete 'configure))
       #:make-flags
       (list (string-append "prefix=" %output))))
    (home-page "http://www.figlet.org/")
    (synopsis "Make large letterforms out of ordinary screen characters")
    (description "FIGlet is a program for making large ASCII art letterforms
out of ordinary screen characters.")
    (license license:bsd-3)))
