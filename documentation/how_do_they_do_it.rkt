#lang racket

(+ 1
   2
   (call-with-continuation-prompt
    (lambda ()
      (call-with-composable-continuation
       (lambda (k)
         (k 7))))))
(call-with-continuation-prompt
 (lambda ()
   (+ 1
      2
      (call-with-composable-continuation
       (lambda (k)
         (k 7))))))
(define x 5)
(define tag (make-continuation-prompt-tag 'my-tag))
(call-with-continuation-prompt
 (lambda ()
   (abort-current-continuation tag))
 tag
 (lambda ()
   (displayln x)
   (set! x (sub1 x))
   (abort-current-continuation tag)))