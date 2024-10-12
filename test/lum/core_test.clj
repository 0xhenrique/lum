(ns lum.core-test
  (:require [clojure.test :refer :all]
            [lum.core :refer :all]
            [cheshire.core :as json]
            [clojure.java.io :as io]))

(def sample-bookmark-data
  {:owner "admin"
   :created-at 1633036800
   :bookmarks [{:link "https://example.com"
                :created-at 1633036800
                :last-updated 1633036800}]})

(deftest test-generate-lum
  (testing "Generate the bookmark file"
    (let [home (home-dir)
          file-path (str home "/bookmarks/lum-marker.json")]
      (generate-lum)
      (is (.exists (io/file file-path)))
      (let [data (-> (slurp file-path) (json/parse-string true))]
        (is (= (:owner data) "user"))))))
