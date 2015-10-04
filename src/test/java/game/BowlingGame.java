package game;

import org.junit.Before;
import org.junit.Ignore;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.MatcherAssert.assertThat;

public class BowlingGame {

    private Game game;

    @Before
    public void setUp() throws Exception {
        game = new Game();
    }

    @Test
    public void testRollBall() throws Exception {
        game.rollBall(0);
    }

    @Test
    public void testGutterGame_scoreShouldBeZero() throws Exception {
        for (int i = 0; i < 20; i++) {
            game.rollBall(0);
        }
        assertThat(game.score(), is(0));
    }

    @Test
    public void testAllTriesIsOne_scoreShouldBeTwenty() throws Exception {
        for (int i = 0; i < 20; i++) {
            game.rollBall(1);
        }
        assertThat(game.score(), is(20));
    }
}
