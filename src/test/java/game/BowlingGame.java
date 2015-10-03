package game;

import org.junit.Before;
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
        for (int i = 0; i < 10; i++) {
            game.rollBall(0);
        }
        assertThat(game.score(), is(0));
    }
}
