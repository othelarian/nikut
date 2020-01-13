# TODO LIST :

## DAY 1 (2020/01/05)

* open a glutin window => DONE
* test glutin multiple windows => ABORTED(useless)
* try luminance => DONE
* change background color from input (keyboard) => DONE
* try luminance with multiple windows => WIP
  * rewrite luminance_glutin => WIP
  * check if inputs are window relative
* recode the Nuklear example => WIP
  * struct nk_context ctx? (found) => DONE
  * nk_init_fixed?

## DAY 2 (2020/01/07)

* do the hello world of luminance without the luminance-glutin => WIP
  * get the main win => DONE
  * change the background color (with refresh) => DONE
  * integrate input to change bg color => DONE
  * try add some gl from hello_world_glutin.rs => DONE
  * move the window in a hashmap kind of struct => ABORTED
  * launch three windows (connected to the hasmap) =>ABORTED
  * change the version of glutin (new api!) => WIP

## DAY 3 (2020/01/08)

* do the hello_world_glutin.rs with new glutin vers (from yesterday) and deviate to multiple windows => WIP
  * make it work correctly with the new glutin version => DONE
  * make one window with the ContextTracker => WIP
  * launch 3 windows in parallel

## DAY 4 (2020/01/09)

* finish the day 4 goals => WIP
  * use the context tracker for managing opengl and surface => ABORTED
  * construct a window manager, who take care of opengl context internally => DONE
  * test the window manager with one window, fully featured => WIP
  * launch 3 windows in parallel

## DAY 5 (2020/01/10)

* always the same, finish the day 4 => WIP
  * go back and try to change from &mut to mut (maybe with takeable) => WIP

## DAY 6 (2020/01/13)

* another day, same activity => WIP
  * finish the winger.rs file, fixing the "get_current" function (last part) => DONE
  * test the first window with the new WinManager => WIP

## PREPARED


* test two windows in parallel

* work on Nuklear Window part

* list all widgets
