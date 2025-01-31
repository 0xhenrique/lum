(ns lum.core
  (:require [clojure.java.io :as io]
            [cheshire.core :as json]
            [clojure.java.shell :as shell]
            [clojure.string :as str])
  (:import (java.util Date)
           (java.nio.file Files Paths)
           (java.nio.file.attribute FileAttribute)))

(defrecord Bookmark [link description created-at last-updated])
(defrecord BookmarkData [owner created-at bookmarks])

(def version "1.0.0")

(defn current-timestamp []
  (quot (.getTime (Date.)) 1000))

(defn home-dir []
  (System/getenv "HOME"))

(defn owner []
  (or (System/getenv "OWNER") "user"))

(defn generate-lum []
  (let [home (home-dir)
        bookmarks-path (str home "/.lum.json")]
    (if (.exists (io/file bookmarks-path))
      (println "The bookmark file already exists at:" bookmarks-path)
      (let [computer-name (owner)
            created-at (current-timestamp)
            bookmark (->Bookmark "https://github.com/0xhenrique/lum" "a short description here..." created-at created-at)
            bookmark-data (->BookmarkData computer-name created-at [bookmark])
            json-data (json/generate-string bookmark-data {:pretty true})]
        (try
          (io/make-parents bookmarks-path)
          (spit bookmarks-path json-data)
          (println "Bookmark file generated at:" bookmarks-path)
          (catch Exception e
            (println "Error creating bookmark file:" (.getMessage e))))))))

(defn print-lum-version []
  (println "Lum version:" version))

(defn view-bookmarks []
  (let [file-path (str (home-dir) ".lum.json")]
    (try
      (let [bookmark-data (-> (slurp file-path) (json/parse-string true))]
        (doseq [bookmark (:bookmarks bookmark-data)]
          (println (:link bookmark))))
      (catch Exception e
        (println "Failed to view bookmarks:" (.getMessage e))))))

(defn detail-bookmarks []
  (let [file-path (str (home-dir) ".lum.json")]
    (try
      (let [bookmark-data (-> (slurp file-path) (json/parse-string true))]
        (doseq [[i bookmark] (map-indexed vector (:bookmarks bookmark-data))]
          (println (str (inc i) ". - " (:link bookmark)))))
      (catch Exception e
        (println "Failed to view bookmarks:" (.getMessage e))))))

(defn print-help []
  (println "
Lum - Lum Universal Marker

Just for fun.

Usage: lum [OPTION] value

OPTIONS:
  -l, --list      -    Lists all available bookmarks
  -v, --version   -    Prints the current version of Lum
  -h, --help      -    Shows this help output
  -g, --generate  -    Generate the Bookmark file
  -a, --add       -    Add a new bookmark to an already existent bookmark file
  -d, --delete    -    Delete a bookmark by its index"))

(defn add-new-bookmark [link]
  (let [file-path (str (home-dir) ".lum.json") 
        created-at (current-timestamp)
        last-updated created-at]

    (try
      (let [bookmark-data (-> (slurp file-path) (json/parse-string true))
            new-bookmark (->Bookmark link "" created-at last-updated)
            updated-bookmark-data (update bookmark-data :bookmarks conj new-bookmark)
            updated-json (json/generate-string updated-bookmark-data {:pretty true})]

        (spit file-path updated-json)
        (println "New bookmark added successfully."))

      (catch Exception e
        (println "Failed to add new bookmark:" (.getMessage e))))))

(defn delete-bookmark [index-str]
  (let [index (Integer/parseInt index-str)
        file-path (str (home-dir) ".lum.json")]

    (try
      (let [bookmark-data (-> (slurp file-path) (json/parse-string true))]
        (if (< index (count (:bookmarks bookmark-data)))
          (let [updated-bookmark-data (assoc bookmark-data :bookmarks
                                             (vec (concat (subvec (:bookmarks bookmark-data) 0 index)
                                                          (subvec (:bookmarks bookmark-data) (inc index)))))
                updated-json (json/generate-string updated-bookmark-data {:pretty true})]

            (spit file-path updated-json)
            (println "Bookmark deleted successfully."))
          (println "Invalid index. Bookmark does not exist.")))
      (catch Exception e
        (println "Failed to delete bookmark:" (.getMessage e))))))

(defn parse-cli-args [args]
  (cond
    (or (empty? args) (= "-h" (first args)) (= "--help" (first args)))
    (print-help)

    (or (= "-l" (first args)) (= "--list" (first args)))
    (view-bookmarks)

    (or (= "-v" (first args)) (= "--version" (first args)))
    (print-lum-version)

    (or (= "-g" (first args)) (= "--generate" (first args)))
    (generate-lum)

    (or (= "-a" (first args)) (= "--add" (first args)))
    (when (second args)
      (add-new-bookmark (second args)))

    (or (= "-d" (first args)) (= "--delete" (first args)))
    (when (second args)
      (delete-bookmark (second args)))

    :else
    (println "Invalid option. Use -h to see available options.")))

(defn -main
  "Lum Universal Marker"
  [& args]
  (parse-cli-args args))
